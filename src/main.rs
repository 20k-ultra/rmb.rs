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
    // TODO handle config file missing (this will panick on first run since the file won't exist)
    let data = fs::read_to_string(data_path).unwrap();
    // Parse CLI arguments provided
    let args = Cli::parse();
    // Process command provided
    let modified_data = handle_cmd(&args.command, &data);
    // Check if data was modified
    if modified_data.is_some() {
        // Persist edit
        fs::write(data_path, modified_data.unwrap()).expect("Unable to write file")
    }
}

fn handle_cmd(cmd: &Commands, data: &String) -> Option<String> {
    match cmd {
        Commands::Link { link } => Some(rmbrs::link::add(
            &rmbrs::link::Link {
                url: link.to_string(),
            },
            &data,
        )),
        Commands::Todo { todo } => Some(rmbrs::todo::add(
            &rmbrs::todo::Todo {
                task: todo.to_string(),
            },
            &data,
        )),
        Commands::Timer { when, what } => Some(rmbrs::timer::add(
            &rmbrs::timer::Timer {
                when: when.to_string(),
                what: what.to_string(),
            },
            &data,
        )),
        Commands::List {} => {
            rmbrs::print(&data);
            None
        }
        Commands::Links {} => {
            rmbrs::link::print(&data);
            None
        }
        Commands::Todos {} => {
            rmbrs::todo::print(&data);
            None
        }
        Commands::Timers {} => {
            rmbrs::timer::print(&data);
            None
        }
    }
}
