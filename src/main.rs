use clap::{command, value_parser, Arg, ArgAction, ArgMatches};
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Stdin};
use std::path::PathBuf;

fn main() {
    let matches: ArgMatches = command!()
        .author("Maurice-Jörg Nießen <info@mjniessen.com>")
        .arg(Arg::new("FILE").value_parser(value_parser!(PathBuf)))
        .arg(
            Arg::new("COUNT")
                .short('c')
                .long("count")
                .default_value("1")
                .value_parser(value_parser!(u16).range(1..))
                .help("Repeat picking <COUNT> times"),
        )
        .arg(
            Arg::new("UNIQUE")
                .short('u')
                .long("unique")
                .action(ArgAction::SetTrue)
                .help("Unique selection of lines"),
        )
        .get_matches();

    let mut count: u16 = 1;

    if matches.contains_id("COUNT") {
        let given_count = matches.get_one::<u16>("COUNT").unwrap();
        count = *given_count;
    }

    // TODO: Check, if file exists
    if matches.contains_id("FILE") {
        let input = matches.get_one::<PathBuf>("FILE").unwrap();
        let lines = lines_from_file(input);
        pick_repeat(count as usize, lines);
    } else if atty::isnt(atty::Stream::Stdin) {
        let input = stdin();
        let lines = lines_from_stdin(input).unwrap();
        pick_repeat(count as usize, lines);
    }
}

fn lines_from_stdin(stdin: Stdin) -> std::io::Result<Vec<String>> {
    let mut lines = stdin.lock().lines();
    let mut res = Vec::new();
    for line in &mut lines {
        let line = line?;
        let items = line.split('\n').collect();
        res.push(items);
    }
    Ok(res)
}

// TODO: change to std::io::Result<Vec<String>>
fn lines_from_file(filename: &PathBuf) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn random(max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max)
}

// TODO: unique lines
fn pick_repeat(amount: usize, lines: Vec<String>) {
    let max = lines.len();
    let mut count = amount;
    while count > 0 {
        println!("{}", lines[random(max)]);
        count -= 1;
    }
}
