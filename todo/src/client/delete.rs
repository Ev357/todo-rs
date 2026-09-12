use color_eyre::eyre::Result;
use uuid::Uuid;

use crate::client::ApiClient;

pub async fn delete_todo(id: Uuid, api: ApiClient) -> Result<()> {
    let _ = api
        .client
        .delete(api.url(&format!("/api/todo/{id}")))
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
