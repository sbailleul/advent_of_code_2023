use std::fmt::{write, Display};
#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}
impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "time: {}, distance: {}", self.time, self.distance)
    }
}

impl Race {
    fn compute_ways_to_win_count(&self) -> u32 {
        let mut ways_to_win = 0;
        for i in 1..self.time - 1 {
            let distance = Self::get_distance_by_time(&i, &self.time);
            if distance > self.distance {
                ways_to_win += 1;
            }
        }
        ways_to_win
    }

    fn get_distance_by_time(hold_time: &u32, max_time: &u32) -> u32 {
        hold_time * (max_time - hold_time)
    }
}
struct LeaderBoard(Vec<Race>);

impl From<&str> for LeaderBoard {
    fn from(value: &str) -> Self {
        let lines = value
            .lines()
            .map(|l| get_line_values(l))
            .collect::<Vec<Vec<u32>>>();
        Self(
            lines[0]
                .iter()
                .zip(lines[1].iter())
                .map(|(&time, &distance)| Race { time, distance })
                .collect(),
        )
    }
}

impl LeaderBoard {
    fn get_total_ways_to_win(&self) -> u32 {
        self.0
            .iter()
            .map(|r| r.compute_ways_to_win_count())
            .reduce(|c1, c2| c1 * c2)
            .unwrap()
    }
}

fn get_line_values(line: &str) -> Vec<u32> {
    let (_, values) = line.split_once(':').unwrap();
    values
        .split(' ')
        .map(|v| v.parse::<u32>())
        .filter_map(|r| r.ok())
        .collect()
}
pub fn step_1(input_content: &str) -> String {
    let leaderboard = LeaderBoard::from(input_content);
    leaderboard.get_total_ways_to_win().to_string()
}

pub fn step_2(input_content: &str) -> String {
    "0".to_string()
}

#[cfg(test)]
mod tests {
    use super::step_1;

    const INPUT_CONTENT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;
    #[test]
    fn step_1_should_works() {
        let res = step_1(INPUT_CONTENT);
        assert_eq!(res, "288");
    }
}
