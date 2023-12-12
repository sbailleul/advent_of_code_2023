#![feature(test)]

use std::{
    collections::{HashMap, VecDeque},
    env, fs,
};

use clap::Parser;

extern crate test;
mod challenge_1;
mod challenge_2;
mod challenge_8;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value_t=1)]
    step: u8,
    #[arg(short, long)]
    challenge_id: u8,
    #[arg(short, long)]
    file_path: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();

    let input_content = fs::read_to_string(args.file_path).unwrap();
    let challenges = HashMap::from([
        ((8, 1), challenge_8::step as fn(&str)),
        ((1, 1), challenge_1::step_1 as fn(&str)),
        ((1, 2), challenge_1::step_2 as fn(&str)),
    ]);
    challenges[&(args.challenge_id, args.step)](&input_content);
}
