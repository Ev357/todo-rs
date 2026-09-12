use color_eyre::eyre::Result;
use todo_server::db::todo::CreateTodo;

use crate::client::ApiClient;

pub async fn post_todo(data: CreateTodo, api: ApiClient) -> Result<()> {
    let _ = api
        .client
        .post(api.url("/api/todo"))
        .json(&data)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
