use clap::ArgMatches;

use std::fs;
use std::str::FromStr;

mod cli;
mod rmbrs;

fn main() {
    // TODO allow specifying data store location
    let data_path = match home::home_dir() {
        Some(path) => format!("{}/.rmbrs.json", path.display()), 
        None => exit_msg("Unable to look up HOME directory", exitcode::CONFIG), 
    };
    // Get existing data or return default CLI data
    let mut remembered = match fs::read_to_string(&data_path) {
        Ok(d) => rmbrs::Remembers::from_str(&d).expect("Remembered data is corrupt"),
        Err(_) => rmbrs::Remembers::new(),
    };
    // Parse CLI arguments provided
    let matches = cli::build_cli().get_matches();
    let cmd = match matches.subcommand() {
        Some(sub) => {
            (cli::SubCommand::from_str(sub.0).unwrap(), sub.1)
        },
        None => {
            unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`");
        }
    };
    // Check if something was changed
    if handle_cmd(cmd, &mut remembered) {
        // Persist change
        fs::write(data_path, remembered.to_string()).expect("Unable to write file")
    }
}

// TODO use clap lib Error
fn exit_msg(message: &str, code: i32) -> ! {
    eprintln!("{message}");
    std::process::exit(code);
}

fn handle_cmd((cmd, args): (cli::SubCommand, &ArgMatches), data: &mut rmbrs::Remembers) -> bool {
    match (cmd, args.subcommand()) {
        (cli::SubCommand::Print, _) => {
            // TODO make this prettier like a table with colours...
            println!(
                "Remembers\n   Links\n{}\n   Todos\n{}\n   Timers\n{}",
                data.links, data.todos, data.timers
            );
            false
        }
        (cli::SubCommand::Links, sub_cmd) => {
            match sub_cmd {
                Some(more_sub_cmd) => {
                    match (more_sub_cmd.0, more_sub_cmd.1) {
                        ("add", aargs) => {
                            data.links.add(aargs.value_of("link").unwrap().to_string());
                            println!("Added link");
                            true
                        }
                        ("rm", aargs) => {
                            let index = aargs.value_of("index").unwrap();
                            data.links.remove(index.parse::<usize>().unwrap() - 1);
                            println!("Removed link");
                            true
                        }
                        _ => {
                            unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`");
                        }
                    }
                },
                None => {
                    println!("{}", data.links);
                    false
                }
            }
        }
        (cli::SubCommand::Todos, sub_cmd) => {
            match sub_cmd {
                Some(more_sub_cmd) => {
                    match (more_sub_cmd.0, more_sub_cmd.1) {
                        ("add", aargs) => {
                            data.todos.add(aargs.value_of("todo").unwrap().to_string());
                            println!("Added todo");
                            true
                        }
                        ("rm", aargs) => {
                            let index = aargs.value_of("index").unwrap();
                            data.todos.remove(index.parse::<usize>().unwrap() - 1);
                            println!("Removed todo");
                            true
                        }
                        _ => {
                            unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`");
                        }
                    }
                },
                None => {
                    println!("{}", data.todos);
                    false
                }
            }
        }
        (cli::SubCommand::Timers, sub_cmd) => {
            match sub_cmd {
                Some(more_sub_cmd) => {
                    match (more_sub_cmd.0, more_sub_cmd.1) {
                        ("add", aargs) => {
                            let what = aargs.value_of("what").unwrap().to_string();
                            let when = aargs.value_of("when").unwrap().to_string();
                            data.timers.add(what, when);
                            println!("Added timer");
                            true
                        }
                        ("rm", aargs) => {
                            let index = aargs.value_of("index").unwrap();
                            data.timers.remove(index.parse::<usize>().unwrap() - 1);
                            println!("Removed link");
                            true
                        }
                        _ => {
                            unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`");
                        }
                    }
                },
                None => {
                    println!("{}", data.timers);
                    false
                }
            }
        }
    }
}
