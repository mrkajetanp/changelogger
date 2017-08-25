extern crate changelogger;
extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("changelogger")
        .version("0.1")
        .author("Cajetan Puchalski <cajetan.puchalski@gmail.com>")
        .about("A simple tool for updating changelogs")
        .arg(Arg::with_name("add_exercise")
             .short("e")
             .long("add_exercise")
             .value_name("LANGUAGE")
             .value_name("TITLE")
             .value_name("SOURCE")
             .help("Adds a new programming exercise")
        .takes_value(true))
        .arg(Arg::with_name("add_commit")
             .short("c")
             .long("add_commit")
             .value_name("CATEGORY")
             .value_name("DESCRIPTION")
             .help("Adds a new development project commit")
        .takes_value(true))
        .arg(Arg::with_name("file")
             .short("f")
             .value_name("PATH")
             .multiple(false)
             .help("Path to the CHANGELOG file"))
        .get_matches();

    if let Some(exercise) = matches.values_of("add_exercise") {
        let data: Vec<&str> = exercise.collect();

        let path = match matches.value_of("file") {
            Some(s) => s,
            None => "CHANGELOG.md",
        };

        if let Err(e) = changelogger::add_exercise(data[0], data[1], data[2], path) {
            println!("{}", e);
        }
    }

    if let Some(commit) = matches.values_of("add_commit") {
        let data: Vec<&str> = commit.collect();

        if let Err(e) = changelogger::add_commit(data[0], data[1]) {
            println!("{}", e);
        }
    }
}
