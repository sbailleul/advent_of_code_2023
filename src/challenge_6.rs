use std::{
    borrow::Borrow,
    fmt::{write, Display},
    iter,
};
#[derive(Debug, Clone)]
struct Race {
    time: u64,
    distance: u64,
}
impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "time: {}, distance: {}", self.time, self.distance)
    }
}

impl<'a> iter::Sum<&'a Self> for Race {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(
            Race {
                distance: 0,
                time: 0,
            },
            |race1, race2| {
                let res = Race {
                    time: (race1.time.to_string() + &race2.time.to_string())
                        .parse()
                        .unwrap(),
                    distance: (race1.distance.to_string() + &race2.distance.to_string())
                        .parse()
                        .unwrap(),
                };
                res
            },
        )
    }
}

impl Race {
    fn compute_ways_to_win_count(&self) -> u64 {
        let mut ways_to_win = 0;
        for i in 1..self.time - 1 {
            let distance = Self::get_distance_by_time(&i, &self.time);
            if distance > self.distance {
                ways_to_win += 1;
            }
        }
        ways_to_win
    }

    fn get_distance_by_time(hold_time: &u64, max_time: &u64) -> u64 {
        hold_time * (max_time - hold_time)
    }
}
struct LeaderBoard(Vec<Race>);

impl From<&str> for LeaderBoard {
    fn from(value: &str) -> Self {
        let lines = value
            .lines()
            .map(|l| get_line_values(l))
            .collect::<Vec<Vec<u64>>>();
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
    fn get_total_ways_to_win(&self) -> u64 {
        self.0
            .iter()
            .map(|r| r.compute_ways_to_win_count())
            .reduce(|c1, c2| c1 * c2)
            .unwrap()
    }
    fn get_total_ways_to_win_for_aggregated_races(&self) -> u64 {
        self.0.iter().sum::<Race>().compute_ways_to_win_count()
    }
}

fn get_line_values(line: &str) -> Vec<u64> {
    let (_, values) = line.split_once(':').unwrap();
    values
        .split(' ')
        .map(|v| v.parse::<u64>())
        .filter_map(|r| r.ok())
        .collect()
}
pub fn step_1(input_content: &str) -> String {
    let leaderboard = LeaderBoard::from(input_content);
    leaderboard.get_total_ways_to_win().to_string()
}

pub fn step_2(input_content: &str) -> String {
    let leaderboard = LeaderBoard::from(input_content);
    leaderboard
        .get_total_ways_to_win_for_aggregated_races()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_CONTENT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;
    #[test]
    fn step_1_should_works() {
        let res = step_1(INPUT_CONTENT);
        assert_eq!(res, "288");
    }

    #[test]
    fn step_2_should_works() {
        let res = step_2(INPUT_CONTENT);
        assert_eq!(res, "71503");
    }
}
