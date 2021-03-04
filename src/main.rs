extern crate exitcode;
extern crate ini;

use clap::{crate_version, App, Arg, ArgMatches};
use ini::Ini;
use std::io;

// This is bad right?
const INPUT: &'static str = "input";
const SECTION: &'static str = "section";
const KEY: &'static str = "key";
const VALUE: &'static str = "value";

fn main() {
    let matches = App::new("clini")
        .about("Basic ini file manipulation.")
        .author("mail@domgreen.com")
        .version(crate_version!())
        .subcommand(
            App::new("get").args(&[
                Arg::new(INPUT).takes_value(true).required(true),
                Arg::new(SECTION)
                    .short('s')
                    .long(SECTION)
                    .takes_value(true)
                    .required(true),
                Arg::new(KEY)
                    .short('k')
                    .long(KEY)
                    .takes_value(true)
                    .required(true),
            ]),
        )
        .subcommand(
            App::new("set").args(&[
                Arg::new(INPUT).takes_value(true).required(true),
                Arg::new(SECTION)
                    .short('s')
                    .long(SECTION)
                    .takes_value(true)
                    .required(true),
                Arg::new(KEY)
                    .short('k')
                    .long(KEY)
                    .takes_value(true)
                    .required(true),
                Arg::new(VALUE)
                    .short('v')
                    .long(VALUE)
                    .takes_value(true)
                    .required(true),
            ]),
        )
        .subcommand(
            App::new("del").args(&[
                Arg::new(INPUT).takes_value(true).required(true),
                Arg::new(SECTION)
                    .short('s')
                    .long(SECTION)
                    .takes_value(true)
                    .required(true),
                Arg::new(KEY)
                    .short('k')
                    .long(KEY)
                    .takes_value(true)
                    .required(true),
            ]),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("get", get_matches)) => match get_val(get_matches) {
            None => {
                std::process::exit(exitcode::UNAVAILABLE);
            }
            Some(val) => println!("{}", val),
        },
        Some(("set", set_matches)) => {
            set_val(set_matches).unwrap_or_else(|error| {
                println!("Error: {}", error);
            });
        }
        Some(("del", del_matches)) => {
            del_value(del_matches).unwrap_or_else(|error| {
                println!("Error: {}", error);
            });
        }
        None => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    std::process::exit(exitcode::OK);
}

fn set_val(set_matches: &ArgMatches) -> Result<(), io::Error> {
    let input: &str = set_matches.value_of(INPUT).unwrap();
    let section: &str = set_matches.value_of(SECTION).unwrap();
    let param: &str = set_matches.value_of(KEY).unwrap();
    let val: &str = set_matches.value_of(VALUE).unwrap();

    let mut conf: Ini = Ini::load_from_file(input).unwrap();
    conf.with_section(Some(section)).set(param, val);
    conf.write_to_file(input)
}

fn get_val(get_matches: &ArgMatches) -> Option<String> {
    let input: &str = get_matches.value_of(INPUT)?;
    let section: &str = get_matches.value_of(SECTION)?;
    let key: &str = get_matches.value_of(KEY)?;

    let conf: Ini = Ini::load_from_file(input.to_string()).unwrap_or_else(|error| {
        println!("Error: {}", error);
        std::process::exit(exitcode::DATAERR);
    });
    let value: &str = &conf.section(Some(section))?.get(key)?;
    Some(value.to_string())
}

fn del_value(del_matches: &ArgMatches) -> Result<(), io::Error> {
    let input: &str = del_matches.value_of(INPUT).unwrap();
    let section: &str = del_matches.value_of(SECTION).unwrap();
    let key: &str = del_matches.value_of(KEY).unwrap();

    let mut conf: Ini = Ini::load_from_file(input).unwrap();
    conf.with_section(Some(section)).delete(&key);
    conf.write_to_file(input)
}
