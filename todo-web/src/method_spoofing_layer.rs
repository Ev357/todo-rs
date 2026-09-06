use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use dioxus::{
    fullstack::response::IntoResponse,
    server::axum::{
        body::{to_bytes, Body},
        extract::Request,
        http::{header, Method, StatusCode},
        response::Response,
    },
};
use tower::{Layer, Service};

const MAX_FORM_BYTES: usize = 2 * 1024 * 1024;

#[derive(Clone)]
pub struct MethodSpoofingLayer;

impl<S> Layer<S> for MethodSpoofingLayer {
    type Service = MethodSpoofingService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        MethodSpoofingService { inner }
    }
}

#[derive(Clone)]
pub struct MethodSpoofingService<S> {
    inner: S,
}

impl<S> Service<Request> for MethodSpoofingService<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            if req.method() != Method::POST {
                return inner.call(req).await;
            }

            let is_form = req
                .headers()
                .get(header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .is_some_and(|content_type| {
                    content_type
                        .to_ascii_lowercase()
                        .starts_with("application/x-www-form-urlencoded")
                });

            if !is_form {
                return inner.call(req).await;
            }

            let (mut parts, body) = req.into_parts();
            let bytes = match to_bytes(body, MAX_FORM_BYTES).await {
                Ok(bytes) => bytes,
                Err(_) => {
                    let res = Response::builder()
                        .status(StatusCode::PAYLOAD_TOO_LARGE)
                        .body(Body::empty())
                        .unwrap_or_else(|_| StatusCode::PAYLOAD_TOO_LARGE.into_response());
                    return Ok(res);
                }
            };

            let pairs: Vec<(String, String)> =
                serde_urlencoded::from_bytes(&bytes).unwrap_or_default();

            let mut overridden_method = None;
            let mut payload_map = serde_json::Map::new();

            for (key, val) in pairs {
                if key == "_method" {
                    overridden_method = Method::from_bytes(val.trim().as_bytes()).ok();
                    continue;
                }

                let json_val = match val.as_str() {
                    "on" => serde_json::Value::Bool(true),
                    s => serde_json::from_str(s).unwrap_or(serde_json::Value::String(val)),
                };

                if let Some((root, subkey)) = key.split_once('[') {
                    let clean_sub = subkey.trim_end_matches(']');

                    let target_object = payload_map
                        .entry(root.to_string())
                        .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));

                    if let serde_json::Value::Object(ref mut map) = target_object {
                        map.insert(clean_sub.to_string(), json_val);
                    }
                } else {
                    payload_map.insert(key, json_val);
                }
            }

            let payload = serde_json::Value::Object(payload_map);
            let json_bytes = match serde_json::to_vec(&payload) {
                Ok(bytes) => bytes,
                Err(_) => {
                    let res = Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response());
                    return Ok(res);
                }
            };

            parts.headers.insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/json"),
            );
            parts.headers.insert(
                header::CONTENT_LENGTH,
                header::HeaderValue::from(json_bytes.len()),
            );

            let referrer = parts.headers.get(header::REFERER).cloned();

            let mut req = Request::from_parts(parts, Body::from(json_bytes));
            if let Some(method) = overridden_method {
                *req.method_mut() = method;
            }

            let response = inner.call(req).await?;

            if is_form && response.status().is_success() {
                if let Some(referrer) = referrer {
                    let mut redirect = Response::builder()
                        .status(StatusCode::SEE_OTHER)
                        .body(Body::empty())
                        .unwrap_or_else(|_| StatusCode::SEE_OTHER.into_response());
                    redirect.headers_mut().insert(header::LOCATION, referrer);
                    return Ok(redirect);
                }
            }

            Ok(response)
        })
    }
}
