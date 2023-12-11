#![feature(test)]

use std::{collections::{VecDeque, HashMap}, env, fs};
extern crate test;
mod challenge_1;
mod challenge_8;


fn main() {
    let mut args = env::args().collect::<VecDeque<String>>();
    let file_path = args.pop_back().unwrap();
    let challenge_id = args.pop_back().unwrap().parse::<u8>().unwrap();
    let input_content = fs::read_to_string(file_path).unwrap();
    let challenges = HashMap::from([
        (8u8, challenge_8::run as fn(&str)),
        (1u8,challenge_1::run as fn(&str))
        ]);
    challenges[&challenge_id](&input_content);
}
