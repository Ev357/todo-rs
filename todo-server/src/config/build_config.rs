#[macro_export]
macro_rules! build_config {
    (
        $(#[$meta:meta])*
        $vis:vis struct $struct_name:ident {
            $( $field_vis:vis $field_name:ident : $field_type:ty $( = $default:expr )? ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $struct_name {
            $( $field_vis $field_name: $field_type ),*
        }

        impl $struct_name {
            $vis async fn load(file_path: &str) -> color_eyre::Result<Self> {
                use std::collections::HashMap;
                use std::env::var;

                use color_eyre::eyre::{eyre, OptionExt};
                use tokio::fs;

                let mut map = HashMap::new();

                if let Ok(file_content) = fs::read_to_string(file_path).await {
                    for line in file_content.lines() {
                        let line = line.trim();

                        if line.is_empty() || line.starts_with('#') {
                            continue;
                        }

                        if let Some((key, value)) = line.split_once('=') {
                            let value = value.trim().trim_matches('"').trim_matches('\'');
                            map.insert(key.trim().to_string(), value.to_string());
                            map.insert(key.trim().to_lowercase(), value.to_string());
                            map.insert(key.trim().to_uppercase(), value.to_string());
                        }
                    }
                }

                Ok(Self {
                    $(
                        $field_name: {
                            let key = stringify!($field_name);
                            let value = var(key)
                                .or_else(|_| var(key.to_uppercase()))
                                .or_else(|_| var(key.to_lowercase()))
                                .ok()
                                .or_else(|| map.get(key).cloned())
                                .or_else(|| map.get(&key.to_uppercase()).cloned())
                                .or_else(|| map.get(&key.to_lowercase()).cloned());

                            $crate::build_config!(@parse_field key, value, $field_type $(, $default)?)
                        },
                    )*
                })
            }
        }
    };

    (@parse_field $key:expr, $value:expr, $field_type:ty, $default:expr) => {
        match $value {
            Some(v) => v.parse::<$field_type>().map_err(|parse_error| {
                eyre!(
                    "Failed to parse key '{}': {}",
                    $key,
                    parse_error
                )
            })?,
            None => $default,
        }
    };

    (@parse_field $key:expr, $value:expr, $field_type:ty) => {
        $value
            .ok_or_eyre(format!("Missing key: {}", $key))?
            .parse::<$field_type>()
            .map_err(|parse_error| {
                eyre!(
                    "Failed to parse key '{}': {}",
                    $key,
                    parse_error
                )
            })?
    };
}
