use color_eyre::eyre::Result;
use reqwest::Method;
use todo_server::db::todo::{QueryTodo, Todo};

use crate::client::ApiClient;

pub async fn query_todo(data: QueryTodo, api: ApiClient) -> Result<Vec<Todo>> {
    let response = api
        .client
        .request(Method::QUERY, api.url("/api/todo"))
        .json(&data)
        .send()
        .await?
        .error_for_status()?;

    let todos = response.json::<Vec<Todo>>().await?;

    Ok(todos)
}
