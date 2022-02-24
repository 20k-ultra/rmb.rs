use clap::{Parser, Subcommand};

use std::fs;

mod rmbrs;

#[derive(Parser)]
#[clap(about = "Command line tool that remembe.rs things", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print everything you wanted to remember
    List {},
    /// Add a URL
    Link {
        /// Link to remember
        link: String,
    },
    /// List URLs
    Links {},
    /// Add a todo list item
    Todo {
        /// Todo item to remember
        todo: String,
    },
    /// List Todo list items
    Todos {},
    /// Add a timer to remind you of something later
    Timer {
        /// When to remind
        when: String,
        /// What to remind
        what: String,
    },
    /// List timers
    Timers {},
}

fn main() {
    // TODO allow specifying data store location
    let data_path = "rmbrs.json";
    // Get existing data or return default CLI data
    let mut remembered = match fs::read_to_string(data_path) {
        Ok(d) => rmbrs::parse(&d).unwrap(), // TODO handle corrupt JSON better
        Err(_) => rmbrs::Remembers::new(),
    };
    // Parse CLI arguments provided
    let args = Cli::parse();
    // Check if something was changed
    if handle_cmd(&args.command, &mut remembered).is_some() {
        // Persist change
        fs::write(data_path, remembered.to_string()).expect("Unable to write file")
    }
}

fn handle_cmd(cmd: &Commands, data: &mut rmbrs::Remembers) -> Option<()> {
    match cmd {
        Commands::Link { link } => {
            data.links.push(rmbrs::Link::new(link.to_owned()));
            Some(())
        }
        Commands::Todo { todo } => {
            data.todos.push(rmbrs::Todo::new(todo.to_owned()));
            Some(())
        }
        Commands::Timer { what, when } => {
            data.timers.push(rmbrs::Timer::new(what.to_owned(), when.to_owned()));
            Some(())
        }
        Commands::List {} => {
            data.print();
            None
        }
        Commands::Links {} => {
            data.links.print();
            None
        }
        Commands::Todos {} => {
            data.todos.print();
            None
        }
        Commands::Timers {} => {
            data.timers.print();
            None
        }
    }
}
