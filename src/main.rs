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
    /// Remove a link
    RmLink {
        /// Link index to remove
        index: usize,
    },
    /// Add a URL
    Link {
        /// Link to remember
        link: String,
    },
    /// List URLs
    Links {},
    /// Remove a todo
    RmTodo {
        /// Todo index to remove
        index: usize,
    },
    /// Add a todo list item
    Todo {
        /// Todo item to remember
        todo: String,
    },
    /// List Todo list items
    Todos {},
    /// Remove a timer
    RmTimer {
        /// Timer index to remove
        index: usize,
    },
    /// Add a timer to remind you of something later
    Timer {
        /// What to remind
        what: String,
        /// When to remind
        when: String,
    },
    /// List timers
    Timers {},
}

fn main() {
    // TODO allow specifying data store location
    let data_path = "rmbrs.json";
    // Get existing data or return default CLI data
    let mut remembered = match fs::read_to_string(data_path) {
        Ok(d) => rmbrs::parse(&d).expect("Remembered data is corrupt"),
        Err(_) => rmbrs::Remembers::new(),
    };
    // Parse CLI arguments provided
    let args = Cli::parse();
    // Check if something was changed
    if handle_cmd(args.command, &mut remembered) {
        // Persist change
        fs::write(data_path, remembered.to_string().unwrap()).expect("Unable to write file")
    }
}

fn handle_cmd(cmd: Commands, data: &mut rmbrs::Remembers) -> bool {
    match cmd {
        Commands::RmLink { index } => {
            data.links.remove(index - 1);
            println!("Removed Link");
            true
        }
        Commands::Link { link } => {
            data.links.add(link);
            println!("Added Link");
            true
        }
        Commands::RmTodo { index } => {
            data.todos.remove(index - 1);
            println!("Removed Todo");
            true
        }
        Commands::Todo { todo } => {
            data.todos.add(todo);
            println!("Added Todo");
            true
        }
        Commands::Timer { what, when } => {
            println!("Will remind you to {what} in {when}");
            data.timers.add(what, when);
            true
        }
        Commands::RmTimer { index } => {
            data.timers.remove(index - 1);
            println!("Removed Timer"); 
            true
        }
        Commands::List {} => {
            println!("{data}");
            false
        }
        Commands::Links {} => {
            println!("{}", data.links);
            false
        }
        Commands::Todos {} => {
            println!("{}", data.todos);
            false
        }
        Commands::Timers {} => {
            println!("{}", data.timers);
            false
        }
    }
}
