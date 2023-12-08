#![feature(test)]

use std::env;
extern crate test;

mod challenge_8;

fn main() {
    let input_file_path = env::args().last().unwrap();
    challenge_8::run(&input_file_path);
}
