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
        Commands::RmLink { index } => {
            data.links.remove(index.to_owned() - 1);
            println!("Removed Link {index}");
            Some(())
        }
        Commands::Link { link } => {
            data.links.add(link.to_owned());
            println!("Added {link}");
            Some(())
        }
        Commands::RmTodo { index } => {
            data.todos.remove(index.to_owned() - 1);
            println!("Removed Todo {index}");
            Some(())
        }
        Commands::Todo { todo } => {
            data.todos.add(todo.to_owned());
            println!("Added {todo}");
            Some(())
        }
        Commands::Timer { what, when } => {
            data.timers.add(what.to_owned(), when.to_owned());
            println!("Will remind you to {what} in {when}");
            Some(())
        }
        Commands::RmTimer { index } => {
            data.timers.remove(index.to_owned() - 1);
            println!("Removed Timer {index}");
            Some(())
        }
        Commands::List {} => {
            println!("{data}");
            None
        }
        Commands::Links {} => {
            println!("{}", data.links);
            None
        }
        Commands::Todos {} => {
            println!("{}", data.todos);
            None
        }
        Commands::Timers {} => {
            println!("{}", data.timers);
            None
        }
    }
}
