use color_eyre::eyre::Result;
use tabled::{builder::Builder, settings::Style};
use todo_server::db::todo::Todo;

pub fn print_todos(todos: &[Todo]) -> Result<()> {
    let mut builder = Builder::default();

    builder.push_record(["id", "title", "is_completed"]);

    for item in todos {
        builder.push_record([
            item.id.to_string(),
            item.title.clone(),
            item.is_completed.to_string(),
        ]);
    }

    let mut table = builder.build();
    table.with(Style::rounded());

    println!("{table}");

    Ok(())
}
