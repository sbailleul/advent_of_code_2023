use std::{
    cell::RefCell,
    fmt::{self, Debug, Display},
    rc::Rc,
    vec,
};

trait Seed: Debug {
    fn match_range(&self, range: &Range) -> bool;
    fn process_range(&mut self, range: &Range);
    fn min(&self) -> u64;
    fn id(&self) -> u8;
}

#[derive(Debug, Clone)]
struct Seeds(Vec<Rc<RefCell<dyn Seed>>>);
#[derive(Debug, Clone)]
struct SimpleSeed(u64, u8);

impl Seed for SimpleSeed {
    fn match_range(&self, range: &Range) -> bool {
        self.0 >= range.source_start && self.0 < range.source_end()
    }

    fn process_range(&mut self, range: &Range) {
        let diff = self.0 - range.source_start;
        self.0 = range.destination_start + diff
    }

    fn min(&self) -> u64 {
        self.0
    }

    fn id(&self) -> u8 {
        self.1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RangedSeed {
    id: u8,
    min: u64,
    max: u64,
}

impl fmt::Display for RangedSeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, min: {}, max: {}",self.id, self.min, self.max)
    }
}
impl RangedSeed {
    fn get_intersection(&self, range: (u64, u64)) -> Self {
        Self {
            id: self.id,
            min: self.min.max(range.0),
            max: self.max.min(range.1),
        }
    }
}
impl Seed for RangedSeed {
    fn match_range(&self, range: &Range) -> bool {
        let (start, end) = (range.source_start, range.source_end());
        let res = self.min >= start && self.max <= end
            || self.min < start && self.max > end
            || self.min < start && self.max >= start
            || self.max > end && self.min <= end;
        if res {
            println!("seed : {} match range: {}", self, range);
        }
        res
    }

    fn process_range(&mut self, range: &Range) {
        let (src_start, src_end, dst_start, dst_end) = (
            range.source_start,
            range.source_end(),
            range.destination_start,
            range.destination_end(),
        );
        let intersection = self.get_intersection((src_start, src_end));
        self.min = dst_start + intersection.min - src_start;
        self.max = dst_end + intersection.max - src_end - 1;
        println!("processed seed {}", self);
    }

    fn min(&self) -> u64 {
        self.min
    }

    fn id(&self) -> u8 {
        todo!()
    }
}

impl Seeds {
    fn from(value: &str, use_ranged_seeds: &bool) -> Self {
        let (_, seeds) = value.split_once(":").unwrap();
        let seeds = seeds
            .trim()
            .split(' ')
            .map(|seed| seed.parse::<u64>())
            .filter_map(|seed| seed.ok())
            .collect::<Vec<u64>>();
        if *use_ranged_seeds {
            Seeds(Self::to_ranged_seeds(&seeds))
        } else {
            Seeds(
                seeds
                    .iter()
                    .enumerate()
                    .map(|(i, &s)| -> Rc<RefCell<dyn Seed>> {
                        Rc::new(RefCell::new(SimpleSeed(s, i as u8)))
                    })
                    .collect(),
            )
        }
    }
    fn to_ranged_seeds(value: &Vec<u64>) -> Vec<Rc<RefCell<dyn Seed>>> {
        let mut ranged_seeds: Vec<Rc<RefCell<dyn Seed>>> = vec![];
        for i in 0..value.len() - 1 {
            if i % 2 == 0 {
                ranged_seeds.push(Rc::new(RefCell::new(RangedSeed {
                    id: (ranged_seeds.len() + 1) as u8,
                    min: value[i],
                    max: value[i] + value[i + 1],
                })));
            }
        }
        ranged_seeds
    }
    fn lowest(&self) -> u64 {
        self.0.iter().map(|s| s.borrow().min()).min().unwrap()
    }
}

struct Almanac<'a> {
    seeds: Seeds,
    maps: Vec<Map<'a>>,
}

impl<'a> Almanac<'a> {
    fn from(value: &'a str, use_ranged_seeds: &bool) -> Self {
        let mut lines = vec![vec![]];
        for line in value.lines() {
            if line.len() == 0 {
                lines.push(vec![]);
            } else {
                let acc = lines.last_mut().unwrap();
                acc.push(line)
            }
        }
        Self {
            seeds: Seeds::from(lines[0][0], use_ranged_seeds),
            maps: lines.iter().skip(1).map(Map::from).collect::<Vec<Map>>(),
        }
    }
    fn process_seeds_to_destination(&self) -> Seeds {
        self.maps
            .iter()
            .fold(self.seeds.clone(), |seeds, map| map.process_seeds(&seeds))
    }
}

#[derive(Debug)]
struct Map<'a> {
    source: &'a str,
    destination: &'a str,
    ranges: Vec<Range>,
}

impl<'a> From<&Vec<&'a str>> for Map<'a> {
    fn from(value: &Vec<&'a str>) -> Self {
        let (source_to_destination, _) = value[0].split_once(' ').unwrap();
        let (source, destination) = source_to_destination.split_once("-to-").unwrap();
        let ranges = value
            .iter()
            .skip(1)
            .map(|&r| Range::from(r))
            .collect::<Vec<Range>>();
        Self {
            source,
            destination,
            ranges,
        }
    }
}

impl Map<'_> {
    fn process_seeds(&self, seeds: &Seeds) -> Seeds {
        println!("Process map {}-to-{}", self.source, self.destination);
        let mut done_seeds= seeds.0.iter().map(|s| false).collect::<Vec<bool>>();
        for range in &self.ranges {
            for (i, seed) in seeds.0.iter().enumerate() {
                if !done_seeds[i] && seed.borrow().match_range(range) {
                    seeds.0[i].borrow_mut().process_range(range);
                    done_seeds[i] = true;
                }
            }
        }
        seeds.clone()
    }
}

#[derive(Debug)]
struct Range {
    destination_start: u64,
    source_start: u64,
    len: u64,
}
impl Range {
    fn source_end(&self) -> u64 {
        self.source_start + self.len
    }
    fn destination_end(&self) -> u64 {
        self.destination_start + self.len
    }
}
impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "source_start: {}, source_end : {}, destination_start: {}, destination_end: {}",
            self.source_start,
            self.source_end(),
            self.destination_start,
            self.destination_end()
        )
    }
}
impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let values = value.split(' ').collect::<Vec<&str>>();
        Self {
            destination_start: values[0].parse().unwrap(),
            source_start: values[1].parse().unwrap(),
            len: values[2].parse().unwrap(),
        }
    }
}

pub fn step_1(input_content: &str) -> String {
    let almanac = Almanac::from(input_content, &false);
    let final_seeds = almanac.process_seeds_to_destination();
    final_seeds.lowest().to_string()
}
pub fn step_2(input_content: &str) -> String {
    let almanac = Almanac::from(input_content, &true);
    let final_seeds = almanac.process_seeds_to_destination();
    final_seeds.lowest().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_CONTENT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn step_1_should_works() {
        let res = step_1(INPUT_CONTENT);
        assert_eq!(res, "35")
    }

    #[test]
    fn step_2_should_works() {
        let res = step_2(INPUT_CONTENT);
        assert_eq!(res, "46")
    }

    #[test]
    fn range_seed_should_be_processable_when_in_range() {
        let range_seed = RangedSeed {id: 1, min: 10, max: 20 };
        let range = Range {
            destination_start: 10,
            source_start: 10,
            len: 10,
        };
        assert!(range_seed.match_range(&range))
    }

    #[test]
    fn range_seed_should_be_processable_when_include_range() {
        let range_seed = RangedSeed {id: 1, min: 10, max: 20 };
        let range = Range {
            destination_start: 15,
            source_start: 15,
            len: 2,
        };
        assert!(range_seed.match_range(&range))
    }

    #[test]
    fn range_seed_should_be_processable_when_start_before_range_and_end_in_range() {
        let range_seed = RangedSeed {id: 1, min: 10, max: 20 };
        let range = Range {
            destination_start: 15,
            source_start: 15,
            len: 10,
        };
        assert!(range_seed.match_range(&range))
    }

    #[test]
    fn range_seed_should_be_processable_when_start_before_range_and_end_after_range() {
        let range_seed = RangedSeed {id: 1, min: 10, max: 40 };
        let range = Range {
            destination_start: 15,
            source_start: 15,
            len: 10,
        };
        assert!(range_seed.match_range(&range))
    }

    #[test]
    fn range_seed_should_be_processable_when_start_after_range_and_end_in_range() {
        let range_seed = RangedSeed {id: 1, min: 20, max: 22 };
        let range = Range {
            destination_start: 15,
            source_start: 15,
            len: 10,
        };
        assert!(range_seed.match_range(&range))
    }

    #[test]
    fn range_seed_should_be_processable_when_start_after_range_and_end_after_range() {
        let range_seed = RangedSeed {id: 1, min: 20, max: 40 };
        let range = Range {
            destination_start: 15,
            source_start: 15,
            len: 10,
        };
        assert!(range_seed.match_range(&range))
    }

    #[test]
    fn range_seed_should_not_be_processable_when_no_intersections_with_range() {
        let range_seed = RangedSeed {id: 1, min: 0, max: 5 };
        let range = Range {
            destination_start: 15,
            source_start: 15,
            len: 10,
        };
        assert!(!range_seed.match_range(&range))
    }

    #[test]
    fn range_process_should_return_new_range_based_on_parent_range() {
        let mut range_seed = RangedSeed {id: 1, min: 10, max: 20 };
        let range = Range {
            destination_start: 20,
            source_start: 8,
            len: 13,
        };
        range_seed.process_range(&range);
        assert_eq!(range_seed, RangedSeed {id: 1, min: 22, max: 32 })
    }
    #[test]
    fn range_process_should_return_new_range_based_on_equivalent_range() {
        let mut range_seed = RangedSeed {id: 1, min: 10, max: 20 };
        let range = Range {
            destination_start: 20,
            source_start: 10,
            len: 10,
        };
        range_seed.process_range(&range);
        assert_eq!(range_seed, RangedSeed {id: 1, min: 20, max: 30 })
    }

    #[test]
    fn range_process_should_return_new_range_based_on_lower_start_and_lower_end_range() {
        let mut range_seed = RangedSeed {id: 1, min: 10, max: 20 };
        let range = Range {
            destination_start: 20,
            source_start: 8,
            len: 10,
        };
        range_seed.process_range(&range);
        assert_eq!(range_seed, RangedSeed {id: 1, min: 22, max: 30 })
    }
}
