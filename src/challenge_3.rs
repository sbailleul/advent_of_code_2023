use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use std::vec;

use crate::matrix::MatrixWrapper;
struct Number {
    value: u32,
    neighbors: HashSet<Neighbor>,
}

impl Number {
    fn get_gears(&self) -> HashSet<&Neighbor> {
        self.neighbors.iter().filter(|n| n.value == '*').collect()
    }
    fn new(digits: &Vec<char>, neighbors: &HashSet<Neighbor>) -> Self {
        Self {
            neighbors: neighbors.clone(),
            value: digits.iter().collect::<String>().parse::<u32>().unwrap(),
        }
    }
    fn is_valid(&self) -> bool {
        let is_valid = self.neighbors.iter().any(Neighbor::is_token);
        if is_valid {
            // println!("VALID {}", self.value)
        } else {
            println!("INVALID {}", self.value)
        }
        is_valid
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Neighbor {
    value: char,
    cell: Cell,
}

impl Neighbor {
    fn is_token(&self) -> bool {
        !self.value.is_digit(10) && self.value != '.'
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Cell(usize, usize);

type Matrix = MatrixWrapper<char>;

impl Matrix {
    fn neighbor_at(&self, row: i32, col: i32) -> Option<Neighbor> {
        self.at(row, col).map(|c| Neighbor {
            cell: Cell(row as usize, col as usize),
            value: c.clone(),
        })
    }
}

fn to_matrix(text: &str) -> Matrix {
    text.lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Matrix>()
}

fn get_neighbors(matrix: &Matrix, row_idx: i32, col_idx: i32) -> HashSet<Neighbor> {
    let mut neighbors = HashSet::new();

    neighbors.insert(matrix.neighbor_at(row_idx - 1, col_idx - 1));
    neighbors.insert(matrix.neighbor_at(row_idx - 1, col_idx));
    neighbors.insert(matrix.neighbor_at(row_idx - 1, col_idx + 1));
    neighbors.insert(matrix.neighbor_at(row_idx, col_idx - 1));
    neighbors.insert(matrix.neighbor_at(row_idx, col_idx + 1));
    neighbors.insert(matrix.neighbor_at(row_idx + 1, col_idx - 1));
    neighbors.insert(matrix.neighbor_at(row_idx + 1, col_idx));
    neighbors.insert(matrix.neighbor_at(row_idx + 1, col_idx + 1));

    neighbors
        .into_iter()
        .filter_map(|n| n)
        .collect::<HashSet<Neighbor>>()
}
fn read_numbers(matrix: &Matrix) -> Vec<Number> {
    let mut neighbors = HashSet::new();
    let mut digits = vec![];
    let mut numbers = vec![];
    for (i, j, c) in matrix.enumerate() {
        if c.is_digit(10) {
            digits.push(*matrix.at(i, j).unwrap());
            neighbors.extend(get_neighbors(matrix, i, j));
        } else if digits.len() > 0 {
            numbers.push(Number::new(&digits, &neighbors));
            digits.clear();
            neighbors.clear();
        }
    }
    numbers
}

pub fn step_1(input_content: &str) -> String {
    let matrix = to_matrix(input_content);
    let numbers = read_numbers(&matrix);
    numbers
        .iter()
        .filter(|&n| n.is_valid())
        .map(|n| n.value as u32)
        .sum::<u32>()
        .to_string()
}
pub fn step_2(input_content: &str) -> String {
    let matrix = to_matrix(input_content);
    let numbers = read_numbers(&matrix);
    let gears_ratio = numbers
        .iter()
        .filter(|&n| n.is_valid())
        .fold(HashMap::new(), |mut numbers_by_gear, number| {
            let gears = number.get_gears();
            for gear in gears {
                let  numbers = numbers_by_gear.entry(gear.cell.clone()).or_insert(Vec::new());
                numbers.push(number.value);
            }
            numbers_by_gear
        })
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum::<u32>();
    gears_ratio.to_string()
}
#[cfg(test)]
mod tests{
    use std::collections::HashSet;

    use crate::challenge_3::{step_1, step_2, to_matrix, get_neighbors, Neighbor, Cell};

    const TEST_INPUT: &str = r#"467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    "#;
    #[test]
    fn step_1_should_works() {
        let res = step_1(TEST_INPUT);
        assert_eq!("4361", res)
    }
    #[test]
    fn step_2_should_works() {
        let res = step_2(TEST_INPUT);
        assert_eq!("467835", res)
    }
    
    #[test]
    fn should_return_neighbors() {
        let matrix = to_matrix(
            r#".....
    .633.
    .#..."#,
        );
        let result = get_neighbors(&matrix, 1, 1);
        let expected_neighbors = HashSet::from([
            Neighbor {
                cell: Cell(0, 0),
                value: '.',
            },
            Neighbor {
                cell: Cell(0, 1),
                value: '.',
            },
            Neighbor {
                cell: Cell(0, 2),
                value: '.',
            },
            Neighbor {
                cell: Cell(1, 0),
                value: '.',
            },
            Neighbor {
                cell: Cell(1, 2),
                value: '3',
            },
            Neighbor {
                cell: Cell(2, 0),
                value: '.',
            },
            Neighbor {
                cell: Cell(2, 1),
                value: '#',
            },
            Neighbor {
                cell: Cell(2, 2),
                value: '.',
            },
        ]);
        assert_eq!(result, expected_neighbors);
    }
    
}
