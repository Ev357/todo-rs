use clap::Parser;
use color_eyre::eyre::Result;
use todo_server::db::todo::{CreateTodo, PatchTodo, QueryTodo};

use crate::{
    cli::{Cli, Commands, print_completions},
    client::{
        ApiClient, delete::delete_todo, get::get_todo, patch::patch_todo, post::post_todo,
        query::query_todo,
    },
    config::Config,
    print_todos::print_todos,
};

mod cli;
mod client;
mod config;
mod print_todos;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    if let Commands::Generate = cli.command {
        print_completions();
        return Ok(());
    }

    let config = Config::new().await;
    let api = ApiClient::new(config.api_url);

    match cli.command {
        Commands::Add { text } => {
            let todo = CreateTodo {
                title: text,
                ..Default::default()
            };
            post_todo(todo, api).await
        }
        Commands::Edit { id, text } => {
            let todo = PatchTodo {
                title: Some(text),
                ..Default::default()
            };
            patch_todo(id, todo, api).await
        }
        Commands::Remove { id } => delete_todo(id, api).await,
        Commands::List => {
            let todos = get_todo(api).await?;
            print_todos(&todos)
        }
        Commands::Search { query } => {
            let query = QueryTodo {
                search: Some(query),
                ..Default::default()
            };
            let todos = query_todo(query, api).await?;
            print_todos(&todos)
        }
        Commands::Generate => unreachable!(),
    }?;

    Ok(())
}
