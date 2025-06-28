use clap::{Parser, Subcommand}; // Import I/O modules for error handling and writing
use crate::tasks::Priority;

#[derive(Parser)]
#[command(name = "todo", version = "1.0", about = "A simple To-Do List Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // Add a new task to the list
    Add { 
        description: String,

        #[arg(short, long)]
        priority: Option<Priority>
    },
    // Mark a task as complete
    Complete { index: usize },
    // Remove task from the list
    Remove { index: usize },
    // List all tasks
    List,
}
