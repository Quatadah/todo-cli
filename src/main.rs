use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use clap::{Parser, Subcommand};
use colored::*;

#[derive(Debug)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String },
    List,
    Complete { id: usize },
    Delete { id: usize },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mut todos: Vec<Todo> = Vec::new();
    let file_path = "todos.txt";

    if Path::new(file_path).exists() {
        load_todos(file_path, &mut todos)?;
    }

    match &cli.command {
        Some(Commands::Add { title }) => add_todo(&mut todos, title)?,
        Some(Commands::List) => list_todos(&todos),
        Some(Commands::Complete { id }) => complete_todo(&mut todos, *id)?,
        Some(Commands::Delete { id }) => delete_todo(&mut todos, *id)?,
        None => println!("No command specified. Use --help for usage information."),
    }

    save_todos(file_path, &todos)?;

    Ok(())
}

fn add_todo(todos: &mut Vec<Todo>, title: &str) -> io::Result<()> {
    let id = todos.len() + 1;
    todos.push(Todo {
        id,
        title: title.to_string(),
        completed: false,
    });

    println!("Todo added successfully!");
    Ok(())
}

fn list_todos(todos: &Vec<Todo>) {
    if todos.is_empty() {
        println!("No todos found.");
    } else {
        for todo in todos {
            let status = if todo.completed {
                "✓".green().to_string()
            } else {
                " ".to_string()
            };
            println!("[{}] {}: {} ({})",
                todo.id,
                status,
                todo.title,
                if todo.completed { "Completed".green() } else { "Pending".yellow() }
            );
        }
    }
}

fn complete_todo(todos: &mut Vec<Todo>, id: usize) -> io::Result<()> {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.completed = true;
        println!("Todo marked as completed!");
    } else {
        println!("Todo with ID {} not found.", id);
    }

    Ok(())
}

fn delete_todo(todos: &mut Vec<Todo>, id: usize) -> io::Result<()> {
    let initial_len = todos.len();
    todos.retain(|t| t.id != id);

    if todos.len() < initial_len {
        println!("Todo deleted successfully!");
    } else {
        println!("Todo with ID {} not found.", id);
    }

    Ok(())
}

fn load_todos(file_path: &str, todos: &mut Vec<Todo>) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 3 {
            todos.push(Todo {
                id: parts[0].parse().unwrap_or(0),
                title: parts[1].to_string(),
                completed: parts[2] == "true",
            });
        }
    }

    Ok(())
}

fn save_todos(file_path: &str, todos: &Vec<Todo>) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    for todo in todos {
        writeln!(file, "{}|{}|{}", todo.id, todo.title, todo.completed)?;
    }

    Ok(())
}