# Rust To-Do List CLI

A simple, fast, and persistent command-line to-do list manager written in Rust. This project was built to demonstrate core Rust concepts including data structures, file I/O, error handling, and application structuring.

---

## Features

- **Add Tasks**: Quickly add new tasks to your list.
- **Complete Tasks**: Mark tasks as complete by their index.
- **Remove Tasks**: Permanently remove tasks from your list.
- **List Tasks**: View all your current tasks, sorted by priority.
- **Priority System**: Assign `low`, `medium`, or `high` priority to tasks.
- **Persistent Storage**: Your to-do list is automatically saved to a `my_todos.json` file, so you never lose your data.
- **Robust CLI**: Built with `clap` for a professional command-line experience with help messages and subcommands.

---

## Installation & Usage

### Prerequisites

You must have the Rust toolchain installed on your system. You can install it from [rustup.rs](https://rustup.rs/).

### Steps

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/your-username/rust-todo-cli.git](https://github.com/your-username/rust-todo-cli.git)
    ```
    *(Remember to replace `your-username` with your actual GitHub username!)*

2.  **Navigate to the project directory:**
    ```bash
    cd rust-todo-cli
    ```

3.  **Run the application using Cargo:**
    All commands follow the pattern `cargo run -- <COMMAND>`.

---

## Commands

### Add a Task

You can add a task with an optional priority flag (`-p` or `--priority`). If no priority is provided, it defaults to `Medium`.

```bash
cargo run -- add "A new task description" --priority high