use clap::{Command, Arg};

use std::str::FromStr;

pub enum SubCommand {
    Todos,
    Links,
    Timers
}

impl FromStr for SubCommand {
    type Err = ();
    fn from_str(input: &str) -> Result<SubCommand, Self::Err> {
        match input {
            "todos"  => Ok(SubCommand::Todos),
            "links"  => Ok(SubCommand::Links),
            "timers" => Ok(SubCommand::Timers),
            _        => Err(()), // TODO improve error messaging when subcommand not found
        }
    }
}

pub fn build_cli() -> Command<'static> {
    Command::new("rmbrs")
        .about("Closest thing to scribbling stuff on a sticky note")
        .subcommand(Command::new("todos")
            .about("Things to do")
            .subcommand(Command::new("add")
                .about("Remember a todo")
                .arg(Arg::new("todo")
                    .takes_value(true)
                    .help("Todo item to add"))
                .arg_required_else_help(true))
            .subcommand(Command::new("rm")
                .about("Forget a todo")
                .arg(Arg::new("index")
                    .takes_value(true)
                    .help("Todo item index remove"))
                .arg_required_else_help(true)))
        .subcommand(Command::new("links")
            .about("Links to remember")
            .subcommand(Command::new("add")
                .about("Remember a link")
                .arg(Arg::new("link")
                    .takes_value(true)
                    .help("Link to add"))
                .arg_required_else_help(true))
            .subcommand(Command::new("rm")
                .about("Forget a link")
                .arg(Arg::new("index")
                    .takes_value(true)
                    .help("Link index to remove"))
                .arg_required_else_help(true)))
        .subcommand(Command::new("timers")
            .about("Things to do at a later time")
            .subcommand(Command::new("add")
                .about("Remember a timer")
                .arg(Arg::new("what")
                    .takes_value(true)
                    .help("What to do"))
                .arg(Arg::new("when")
                    .takes_value(true)
                    .help("When to do it"))
                .arg_required_else_help(true))
            .subcommand(Command::new("rm")
                .about("Forget a timer")
                .arg(Arg::new("index")
                    .takes_value(true)
                    .help("Timer item index remove"))
                .arg_required_else_help(true)))
}

