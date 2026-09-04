use std::str::FromStr;

use color_eyre::eyre::Result;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use tokio::net::TcpListener;

use crate::{api_context::ApiContext, config::Config};

mod api_context;
mod config;
mod db;
mod routes;

const ENV_FILE: &str = ".env";

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let config = Config::load(ENV_FILE).await?;

    let connection_options =
        SqliteConnectOptions::from_str(&config.database_url)?.create_if_missing(true);

    let db = SqlitePool::connect_with(connection_options).await?;

    sqlx::migrate!().run(&db).await?;

    let app = routes::router().with_state(ApiContext { db });

    let listener = TcpListener::bind((config.address, config.port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
