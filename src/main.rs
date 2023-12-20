use std::{collections::HashMap, env, fs};

use clap::Parser;
mod challenge_1;
mod challenge_15;
mod challenge_2;
mod challenge_3;
mod challenge_4;
mod challenge_5;
mod challenge_8;
mod matrix;
#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value_t = 1)]
    step: u8,
    #[arg(short, long)]
    challenge_id: u8,
    #[arg(short, long)]
    file_path: Option<std::path::PathBuf>,
}

fn main() {
    let Cli {
        step,
        challenge_id,
        file_path,
    } = Cli::parse();
    let file_path = file_path.unwrap_or(
        env::current_dir()
            .unwrap()
            .join(format!("inputs/challenge_{challenge_id}.txt")),
    );
    let input_content = fs::read_to_string(file_path).unwrap();
    let challenges = HashMap::from([
        ((1, 1), challenge_1::step_1 as fn(&str) -> String),
        ((1, 2), challenge_1::step_2 as fn(&str) -> String),
        ((2, 1), challenge_2::step_1 as fn(&str) -> String),
        ((2, 2), challenge_2::step_2 as fn(&str) -> String),
        ((3, 1), challenge_3::step_1 as fn(&str) -> String),
        ((3, 2), challenge_3::step_2 as fn(&str) -> String),
        ((4, 1), challenge_4::step_1 as fn(&str) -> String),
        ((4, 2), challenge_4::step_2 as fn(&str) -> String),
        ((5, 1), challenge_5::step_1 as fn(&str) -> String),
        ((5, 2), challenge_5::step_2 as fn(&str) -> String),
        ((8, 1), challenge_8::step as fn(&str) -> String),
        ((15, 1), challenge_15::step_1 as fn(&str) -> String),
    ]);

    if let Some(step_handler) = challenges.get(&(challenge_id, step)) {
        let res = step_handler(&input_content);
        println!("Result for step {step} of challenge {challenge_id} is {res}")
    } else {
        println!("Challenge {challenge_id} or step {step} doesn't exists");
    }
}
