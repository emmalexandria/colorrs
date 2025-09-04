use clap::{Arg, ArgAction, ArgMatches, Command, command, value_parser};
use clap_complete::{Generator, Shell, generate};
use enable_ansi_support::enable_ansi_support;
use rand::seq::IndexedRandom;
use std::{borrow::Cow, ffi::OsStr, path::PathBuf, process::exit};

use crate::{
    download::download_patterns,
    files::list_dir_files,
    patterns::{get_pattern_dir, print_pattern},
};

mod download;
mod files;
mod patterns;

fn main() {
    let cli = build_cli();

    let matches = cli.get_matches_from(wild::args());

    match enable_ansi_support() {
        Ok(()) => {}
        Err(e) => eprintln!("Error enabling ANSI support: {}", e),
    }

    // Get the default dir, and use it if there isn't a custom dir defined

    if let Some(sub) = matches.subcommand_name() {
        // If its the shell generation subcommand, we do that and exit early so we can fetch dir
        // for the rest of the commands to avoid repeating code
        if sub == "generate" {
            completions_cmd(&matches);
        }
        let dir = get_dir(&matches);
        match sub {
            "print" => {
                let pattern = matches
                    .subcommand_matches("print")
                    .unwrap()
                    .get_one::<String>("name")
                    .cloned();

                print(&dir, pattern);
            }
            "list" => {
                let preview = matches
                    .subcommand_matches("list")
                    .unwrap()
                    .get_one::<bool>("preview")
                    .copied()
                    .unwrap_or(false);
                list(&dir, preview);
            }
            "download" => {
                let url = matches
                    .subcommand_matches("download")
                    .unwrap()
                    .get_one::<String>("repository")
                    .cloned()
                    .unwrap();
                if let Err(e) = download_patterns(url, &dir) {
                    eprintln!(
                        "Failed to download patterns with the error: \n {}",
                        e.message
                    )
                }
            }
            &_ => {}
        }
    }
}

fn get_dir(matches: &ArgMatches) -> std::path::PathBuf {
    let default_dir = get_pattern_dir().unwrap();
    let dir = matches
        .get_one::<PathBuf>("patterndir")
        .unwrap_or(&default_dir);

    // Bit of an ugly clone here, but it's not gonna end the world
    dir.clone()
}

fn build_cli() -> Command {
    let shell = command!("generate")
        .about("Generate completions for your shell")
        .arg(
            Arg::new("shell")
                .value_parser(value_parser!(Shell))
                .action(ArgAction::Set)
                .required(true),
        );

    let print = command!("print")
        .about("Print the given pattern, or a random pattern if none is supplied")
        .arg(Arg::new("name").num_args(1).required(false));

    let list = command!("list").about("List available patterns").arg(
        Arg::new("preview")
            .long("preview")
            .short('p')
            .help("Prints all patterns along with their names")
            .action(ArgAction::SetTrue),
    );
    let download = command!("download")
        .about("Download patterns from a git repository")
        .long_about("Download patterns from a git repository. These patterns are looked for in the ./patterns or ./colorscripts subdirectories.
The repository can either be formatted as a full URL, or as <OWNER>/<NAME>, which will be turned into a GitHub url")
    .arg(Arg::new("repository").num_args(1).required(true));

    command!()
        .subcommand(print)
        .subcommand(list)
        .subcommand(download)
        .subcommand(shell)
        .arg(
            Arg::new("patterndir")
                .short('d')
                .value_name("DIRECTORY")
                .long("dir")
                .help("Set a custom directory for pattern description files")
                .value_parser(value_parser!(PathBuf)),
        )
        .subcommand_required(true)
        .about("Generates completiions for your shell")
}

fn completions_cmd(matches: &ArgMatches) {
    if let Some(generator) = matches
        .subcommand_matches("generate")
        .unwrap()
        .get_one::<Shell>("shell")
        .copied()
    {
        let mut cmd = build_cli();
        let output = generate_shell_completions(generator, &mut cmd);

        println!("{output}");
    }
}

fn print(dir: &PathBuf, pattern: Option<String>) {
    if !dir.exists() {
        eprintln!("Pattern directory {} does not exist", dir.to_string_lossy());
        exit(-1);
    }

    let selected_path: Option<PathBuf>;

    if let Some(p) = pattern {
        let mut pattern_path = dir.clone();
        pattern_path.push(&p);

        if !pattern_path.exists() {
            pattern_path = pattern_path.with_extension("toml");
        }

        if !pattern_path.exists() {
            eprintln!(
                "Pattern file for {} not found in {}",
                p,
                dir.to_string_lossy()
            );
            return;
        }
        selected_path = Some(pattern_path);
    } else {
        selected_path = select_random(dir);
    }

    if let Some(path) = selected_path {
        if let Err(e) = print_pattern(&path) {
            eprintln!("Error printing pattern: {}", e)
        };
    } else {
        eprintln!(
            "Failed to select a random path. Are you sure there are paths in ${}?",
            dir.to_string_lossy()
        );
    }
}

fn list(dir: &PathBuf, preview: bool) {
    if !dir.exists() {
        eprintln!("Pattern directory {} does not exist", dir.to_string_lossy());
        exit(-1);
    }

    let files = list_dir_files(dir).unwrap();

    for file in files {
        let ext = file.extension();
        let filename = file.file_stem().unwrap_or(OsStr::new("")).to_string_lossy();
        if preview && let Err(_) = print_pattern(&file) {
            eprintln!(
                "Failed to print preview of {}",
                file.file_name().unwrap_or_default().to_string_lossy()
            )
        }
        print_file(ext, filename);

        // If file has an extension, print it in brackets. Used to distinguish TOML patterns
    }
}

fn print_file(ext: Option<&OsStr>, filename: Cow<'_, str>) {
    if let Some(e) = ext {
        println!("{} ({})", filename, e.to_string_lossy());
        return;
    }

    println!("{}", filename);
}

fn select_random(dir: &PathBuf) -> Option<PathBuf> {
    if !dir.exists() {
        eprintln!("Pattern directory {} does not exist", dir.to_string_lossy());
        exit(-1);
    }

    let files = list_dir_files(dir).unwrap();
    if !files.is_empty() {
        let mut rng = rand::rng();
        let file = files.choose(&mut rng);

        return file.cloned();
    }

    None
}

fn generate_shell_completions<G: Generator>(generator: G, cmd: &mut Command) -> String {
    let mut buf = Vec::new();
    generate(generator, cmd, cmd.get_name().to_string(), &mut buf);

    String::from_utf8_lossy(&buf).to_string()
}
