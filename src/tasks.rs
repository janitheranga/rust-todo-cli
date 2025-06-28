use clap::ValueEnum;
use serde::{Deserialize, Serialize}; // Import serde traits
use std::fs::{self, File}; // Import the file system module
use std::io::{self, Write};

#[derive(ValueEnum, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium, 
    High,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    description: String,
    completed: bool,
    priority: Priority,
}

pub struct TodoList {
    tasks: Vec<Todo>,
}

impl TodoList {
    pub fn new(file_path: &str) -> io::Result<Self> {
        match fs::read_to_string(file_path) {
            Ok(json_string) => {
                let tasks = serde_json::from_str(&json_string).unwrap_or_else(|_| Vec::new());

                Ok(TodoList { tasks })
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(TodoList { tasks: Vec::new() }),
            Err(e) => Err(e),
        }
    }

    pub fn save(&self, file_path: &str) -> io::Result<()> {
        let json_string = serde_json::to_string_pretty(&self.tasks)?;

        let mut file = File::create(file_path)?;
        let _ = file.write_all(json_string.as_bytes());
        Ok(())
    }

    pub fn add_task(&mut self, description: String, priority: Option<Priority>) {
        let task = Todo {
            description,
            completed: false,
            priority: priority.unwrap_or_default()
        };
        self.tasks.push(task);
        println!("Added task: \"{}\"", self.tasks.last().unwrap().description)
    }

    pub fn complete_task(&mut self, index: usize) {
        // `saturating_sub` prevents underflow if index is 0
        if let Some(task) = self.tasks.get_mut(index.saturating_sub(1)) {
            task.completed = true;
            println!("Completed task: \"{}\"", task.description);
        } else {
            println!("Error: Task #{} not found.", index);
        }
    }

    pub fn remove_task(&mut self, index: usize) {
        if index > 0 && index <= self.tasks.len() {
            let removed_task = self.tasks.remove(index - 1);
            println!("Removed task: \"{}\"", removed_task.description);
        } else {
            println!("Error: Task #{} not found.", index);
        }
    }

    pub fn list_tasks(&mut self) {
        println!("\n--- Your To-Do List ---");

        // Sort the tasks by priority in descending order (High first)
        self.tasks.sort_by(|a, b| b.priority.cmp(&a.priority));

        if self.tasks.is_empty() {
            println!("Your list is empty!");
        } else {
            for (index, task) in self.tasks.iter().enumerate() {
                let status = if task.completed { "[x]" } else { "[ ]" };
                println!("{}. {} [{:?}] {}", index + 1, status, task.priority, task.description);
            }
        }
        println!("-----------------------\n");
    }
}
