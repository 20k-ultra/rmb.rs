use clap::{arg, Command, ErrorKind, Parser, Subcommand};

use rmbrs;

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
    /// Manage URLs to keep
    Link {
        /// Link to remember
        link: Option<String>,
    },
    /// Manage items you want to do
    Todo {
        /// Todo item to remember
        todo: Option<String>,
    },
    /// Manage timers that display reminders at a later time
    Timer {
        /// When to remind
        when: Option<String>,
        /// What to remind
        what: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::List {} => {
            rmbrs::print();
        }
        Commands::Link { link } => match link {
            Some(l) => rmbrs::link::add(l),
            None => rmbrs::link::print(),
        },
        Commands::Todo { todo } => match todo {
            Some(t) => rmbrs::todo::add(t),
            None => rmbrs::todo::print(),
        },
        Commands::Timer { when, what } => match (when, what) {
            (Some(w1), Some(w2)) => rmbrs::timer::add(w1, w2),
            (None, None) => rmbrs::timer::print(),
            (_, _) => {
                Command::new("timer")
                    .about("Manage timers that display reminders at a later time")
                    .arg(arg!(<WHEN> "When to remind"))
                    .arg(arg!(<WHAT> "What to remind"))
                    .error(
                        ErrorKind::MissingRequiredArgument,
                        format!("Missing arguments for creating a timer"),
                    )
                    .exit();
            }
        },
    }
}
