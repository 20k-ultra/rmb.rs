use clap::{Parser, Subcommand};

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
    let args = Cli::parse();

    match &args.command {
        Commands::List {} => rmbrs::print(),
        Commands::Link { link } => rmbrs::link::add(link),
        Commands::Links {} => rmbrs::link::print(),
        Commands::Todo { todo } => rmbrs::todo::add(todo),
        Commands::Todos {} => rmbrs::todo::print(),
        Commands::Timer { when, what } => rmbrs::timer::add(when, what),
        Commands::Timers {} => rmbrs::timer::print(),
    }
}
