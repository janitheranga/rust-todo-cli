mod tasks;
mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use std::io;
use tasks::TodoList;

fn main() -> io::Result<()> {
    let file_path = "my_todos.json";
    let mut my_todo_list = TodoList::new(file_path)?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Add { description , priority} => {
            my_todo_list.add_task(description, priority);
            println!("Task added.")
        },
        Commands::Complete { index } => my_todo_list.complete_task(index),
        Commands::Remove { index } => my_todo_list.remove_task(index),
        Commands::List => my_todo_list.list_tasks(),
    }

    my_todo_list.save(file_path)?;
    
    Ok(())
}
