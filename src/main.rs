#![feature(test)]

use std::{collections::VecDeque, env};
extern crate test;

mod challenge_8;


fn main() {
    let mut args = env::args().collect::<VecDeque<String>>();
    let file_path = args.pop_back().unwrap();
    let challenge_id = args.pop_back().unwrap().parse::<u8>().unwrap();
    match challenge_id {
        8 => challenge_8::run(&file_path),
        _ => (),
    }
}
