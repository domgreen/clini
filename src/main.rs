extern crate exitcode;
extern crate ini;

use clap::{Args, Parser, Subcommand};
use ini::Ini;
use std::io;

#[derive(Debug, Parser)]
#[command(author, version, about = "Basic ini file manipulation", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Args)]
struct EntryArgs {
    /// The `[section]` in a file
    #[arg(long, short)]
    section: String,
    /// The `key=` in a file
    #[arg(long, short)]
    key: String,
    /// If escape sequences with `\` should be interpreted or ignored and taken literally
    #[arg(long, short)]
    ignore_escape: bool,
    /// Path to the input file
    input: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Parse a file and extract a value
    Get {
        #[command(flatten)]
        entry: EntryArgs,
    },
    /// Overwrite or add a value
    Set {
        #[command(flatten)]
        entry: EntryArgs,
        #[arg(long, short)]
        value: String,
    },
    /// Delete a value from a file
    Del {
        #[command(flatten)]
        entry: EntryArgs,
    },
}

fn main() {
    let matches = Cli::parse();

    match &matches.command {
        Commands::Get { entry } => match get_val(entry) {
            None => {
                std::process::exit(exitcode::UNAVAILABLE);
            }
            Some(val) => println!("{}", val),
        },
        Commands::Set { entry, value } => set_val(entry, value).unwrap_or_else(|error| {
            println!("Error: {}", error);
        }),
        Commands::Del { entry } => del_val(entry).unwrap_or_else(|error| {
            println!("Error: {}", error);
        }),
    }

    std::process::exit(exitcode::OK);
}

impl EntryArgs {
    fn load(&self) -> Ini {
        let file = if self.ignore_escape {
            Ini::load_from_file_noescape(&self.input)
        } else {
            Ini::load_from_file(&self.input)
        };
        file.unwrap_or_else(|error| {
            println!("Error: {}", error);
            std::process::exit(exitcode::DATAERR);
        })
    }

    fn write(&self, ini: &Ini) -> Result<(), io::Error> {
        if self.ignore_escape {
            ini.write_to_file_policy(&self.input, ini::EscapePolicy::Nothing)
        } else {
            ini.write_to_file(&self.input)
        }
    }
}

fn set_val(entry: &EntryArgs, value: &str) -> Result<(), io::Error> {
    let mut conf: Ini = entry.load();
    conf.with_section(Some(&entry.section))
        .set(&entry.key, value);
    entry.write(&conf)
}

fn get_val(entry: &EntryArgs) -> Option<String> {
    let conf: Ini = entry.load();
    let value: &str = &conf.section(Some(&entry.section))?.get(&entry.key)?;
    Some(value.to_string())
}

fn del_val(entry: &EntryArgs) -> Result<(), io::Error> {
    let mut conf: Ini = entry.load();
    conf.with_section(Some(&entry.section)).delete(&entry.key);
    entry.write(&conf)
}
