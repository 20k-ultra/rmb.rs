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
    // TODO handle config file missing (this will panick on first run since the file won't exist)
    // Read data store
    let data = fs::read_to_string("rmbrs.json").unwrap();
    // Parse CLI arguments provided
    let args = Cli::parse();
    // Process command provided
    match &args.command {
        Commands::List {} => rmbrs::print(&data),
        Commands::Link { link } => rmbrs::link::add(
            &rmbrs::link::Link {
                url: link.to_string(),
            },
            &data,
        ),
        Commands::Links {} => rmbrs::link::print(&data),
        Commands::Todo { todo } => rmbrs::todo::add(
            &rmbrs::todo::Todo {
                task: todo.to_string(),
            },
            &data,
        ),
        Commands::Todos {} => rmbrs::todo::print(&data),
        Commands::Timer { when, what } => rmbrs::timer::add(
            &rmbrs::timer::Timer {
                when: when.to_string(),
                what: what.to_string(),
            },
            &data,
        ),
        Commands::Timers {} => rmbrs::timer::print(&data),
    }
    // TODO take result from add functions and save to file
}
