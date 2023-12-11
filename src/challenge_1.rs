pub fn run(input_content: &str) {
    let res = input_content.lines().map(|line| {
        let first_digit = line.chars().find_map(|c| c.to_digit(10));
        let last_digit = line.chars().rev().find_map(|c| c.to_digit(10));
        dbg!(first_digit, last_digit);
        first_digit.unwrap() * 10 + last_digit.unwrap()
    }).sum::<u32>(); 
    dbg!(res);
}