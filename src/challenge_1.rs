const DIGITS: [DigitValue; 18] = [
    DigitValue("one", 1),
    DigitValue("two", 2),
    DigitValue("three", 3),
    DigitValue("four", 4),
    DigitValue("five", 5),
    DigitValue("six", 6),
    DigitValue("seven", 7),
    DigitValue("eight", 8),
    DigitValue("nine", 9),
    DigitValue("1", 1),
    DigitValue("2", 2),
    DigitValue("3", 3),
    DigitValue("4", 4),
    DigitValue("5", 5),
    DigitValue("6", 6),
    DigitValue("7", 7),
    DigitValue("8", 8),
    DigitValue("9", 9),
];

struct DigitValue(&'static str, u32);



pub fn step_1(input_content: &str)-> String {
    let res = input_content
        .lines()
        .map(|line| {
            let first_digit = line.chars().find_map(|c| c.to_digit(10));
            let last_digit = line.chars().rev().find_map(|c| c.to_digit(10));
            first_digit.unwrap() * 10 + last_digit.unwrap()
        })
        .sum::<u32>();
    res.to_string()
}

#[derive(Debug)]
struct DigitPosition(usize, u32);

fn get_line_digit(text: &str) -> u32 {
    let mut res = DIGITS
        .iter()
        .enumerate()
        .flat_map(|(i, digit)| {
            text.match_indices(digit.0)
                .map(|(pos, _)| DigitPosition(pos, digit.1))
        })
        .collect::<Vec<DigitPosition>>();
    res.sort_by(|d1, d2| d1.0.cmp(&d2.0));
    let first_digit = res.first().unwrap().1;
    let last_digit = res.last().unwrap().1;
    first_digit * 10 + last_digit
}

pub fn step_2(input_content: &str)->String  {
    let res = input_content.lines().map(get_line_digit).sum::<u32>();
    res.to_string()
}
