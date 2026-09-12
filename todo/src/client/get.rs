use color_eyre::eyre::Result;
use todo_server::db::todo::Todo;

use crate::client::ApiClient;

pub async fn get_todo(api: ApiClient) -> Result<Vec<Todo>> {
    let response = api
        .client
        .get(api.url("/api/todo"))
        .send()
        .await?
        .error_for_status()?;

    let todos = response.json::<Vec<Todo>>().await?;

    Ok(todos)
}
