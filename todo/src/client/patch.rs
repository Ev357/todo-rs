use color_eyre::eyre::Result;
use todo_server::db::todo::PatchTodo;
use uuid::Uuid;

use crate::client::ApiClient;

pub async fn patch_todo(id: Uuid, data: PatchTodo, api: ApiClient) -> Result<()> {
    let _ = api
        .client
        .post(api.url(&format!("/api/todo/{id}")))
        .json(&data)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
