use clap::{Arg, Command, command, value_parser};
use std::{fmt::Display, str::FromStr};

#[derive(Clone)]
enum Outputs {
    Test,
}

impl FromStr for Outputs {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "test" => Ok(Self::Test),
            _ => Err(format!("Invalid script name: {}", s)),
        }
    }
}

impl Display for Outputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Outputs::Test => write!(f, "test"),
        }
    }
}

fn main() {
    let cli = build_cli();

    let matches = cli.get_matches_from(wild::args());

    if matches.get_flag("list") {
        println!("List")
    }
}

fn build_cli() -> Command {
    command!()
        .arg(
            Arg::new("exec")
                .short('e')
                .long("exec")
                .value_name("SCRIPT")
                .help("Execute the given colour script")
                .value_parser(value_parser!(Outputs)),
        )
        .arg(
            Arg::new("random")
                .short('r')
                .long("random")
                .help("Choose a random output script")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("List all available colour scripts")
                .action(clap::ArgAction::SetTrue),
        )
}
