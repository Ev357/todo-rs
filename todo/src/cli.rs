use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(name = "todo", version, about = "Yet another todo app.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(alias = "a")]
    Add { text: String },
    #[command(alias = "e")]
    Edit { id: Uuid, text: String },
    #[command(alias = "r")]
    Remove { id: Uuid },
    #[command(alias = "l")]
    List,
    #[command(alias = "s")]
    Search { query: String },
}
