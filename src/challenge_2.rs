use std::fmt::Display;
use std::iter::{Sum};
use std::ops::Add;
const BAG: Set =Set{blue: 14, red:12, green: 13};
#[derive(Debug)]
struct Game{
    id: u16,
    sets: Vec<Set>
}

impl From <&str> for Game{
    fn from(value: &str) -> Self {
        let (game_id, game_content)= value.split_once(':').unwrap();
        let (_, id) = game_id.split_once(' ').unwrap();
        Self{ id: id.parse().unwrap(), sets: game_content.split(';').map(Set::from).collect()}
    }
}


impl Game{
    fn is_valid(&self, set:&Set ) ->bool{
         self.sets.iter().all(|s| s.is_valid(set) )
    }
}
impl <'a> Sum<&'a Game> for u16{
    fn sum<I: Iterator<Item = &'a Game>>(iter: I) -> Self {
         iter.map(|g|g.id).sum()
    }
}
#[derive(Debug, Hash, PartialEq, Eq)]
enum Color{
    BLUE,
    RED,
    GREEN,
}


impl From<&str> for Color{
    fn from(value: &str) -> Self {
        match value.trim(){
            "blue" => Self::BLUE,
            "red" => Self::RED,
            "green" => Self::GREEN,
            _ => panic!()
        }
    }
}
#[derive(Debug)]
struct CubeCount{
    count: u16,
    color: Color
}


impl From<&str> for CubeCount {
    fn from(value: &str) -> Self {
        let (count, color) = value.trim().split_once(' ').unwrap();
        Self{count: count.parse().unwrap(), color: Color::from(color)}
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Set{
    red : u16,
    green: u16,
    blue: u16,
}
impl Set {
    fn is_valid(&self, set: &Set) ->bool{
        self.blue <= set.blue && self.green <= set.green &&  self.red <= set.red
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "blue:{}, red:{}, green: {}", self.blue, self.red, self.green)
    }
}

impl Add for Set{
    type Output = Set;

    fn add(self, rhs: Self) -> Self::Output {
        Self{blue: self.blue + rhs.blue, green: self.green + rhs.green, red: self.red + rhs.red}
    }
}

impl Sum for Set{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(Set::add).unwrap_or_default()
    }
}

impl From<&str> for Set {
    fn from(value: &str) -> Self {
        value.trim().split(',').map(CubeCount::from).fold(Self::default(), |set, cube| {
            match cube.color {
                Color::BLUE => Self{blue: set.blue + cube.count, ..set} ,
                Color::RED => Self{red: set.red + cube.count, ..set} ,
                Color::GREEN => Self{green: set.green + cube.count, ..set} ,
            }
        })
    }
}

pub fn step_1(input_content: &str) -> String {
    let games = input_content.lines().map(Game::from).collect::<Vec<Game>>();
    let res: u16 = games.iter().filter(|g| g.is_valid(&BAG)).sum();
    res.to_string()
}

#[test]
pub fn set_should_be_valid_when_all_color_counts_are_bellow_bag_color_counts(){
    let bag = Set{blue: 19,green: 18, red:16};
    let valid_set = Set{blue: 14,green: 13, red:12};
    assert!(valid_set.is_valid(&bag), "({valid_set}) hasn't valid colors counts for bag ({bag})")
}
#[test]
pub fn set_should_be_valid_when_one_color_count_is_above_bag_color_count(){
    let bag = Set{blue: 19,green: 18, red:16};
    let invalid_set = Set{blue: 20,green: 13, red:12};
    assert!(!invalid_set.is_valid(&bag), "({invalid_set}) hasn't valid colors counts for bag ({bag})")
}