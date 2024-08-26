# Rust Todo App

This is a command-line todo application written in Rust. It demonstrates various Rust concepts and uses external crates for enhanced functionality.

## Features

- Add new todos
- List all todos
- Mark todos as completed
- Delete todos
- Colorful output for better readability
- Persistent storage using a text file

## Dependencies

This project uses the following external crates:

- `clap`: For parsing command-line arguments
- `colored`: For adding color to the console output

## Usage

After building the project, you can use the following commands:

1. Add a new todo:

   ```
   cargo run -- add "Your todo item"
   ```

2. List all todos:

   ```
   cargo run -- list
   ```

3. Complete a todo (replace 1 with the ID of the todo):

   ```
   cargo run -- complete 1
   ```

4. Delete a todo (replace 1 with the ID of the todo):
   ```
   cargo run -- delete 1
   ```

## Building and Running

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository.
3. Navigate to the project directory.
4. Run `cargo build` to build the project.
5. Use the commands described in the Usage section to interact with the app.

## Learning Rust

This project demonstrates several Rust concepts, including:

- Structs and Enums
- Pattern Matching
- Error Handling
- File I/O
- External Crate Usage
- Command-Line Argument Parsing

Feel free to explore the code and modify it to learn more about Rust!
