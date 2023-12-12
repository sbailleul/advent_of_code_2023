use std::{collections::HashMap};

use test::Bencher;


#[derive(Debug)]
enum Direction {
    L,
    R,
}

impl Direction {
    fn choose_node<'a>(&'a self, options: &'a NodeOptions) -> &Node {
        match *self {
            Direction::L => &options.0,
            Direction::R => &options.1,
        }
    }
}
#[derive(Hash, PartialEq, Eq, Debug)]
struct Node<'a>(&'a str);
impl<'a> From<&'a str> for Node<'a> {
    fn from(value: &'a str) -> Self {
        return Node(value.trim());
    }
}

impl Node<'_> {
    fn start() -> Self {
        Node("AAA")
    }
    fn end_node() -> Self {
        Node("ZZZ")
    }
    fn is_end(&self) -> bool {
        self == &Node::end_node()
    }
    fn is_ghost_equivalent(&self, other: &Node<'_>) -> bool {
        self.0[..2] == other.0[..2]
    }
    fn is_ghost_start(&self) -> bool {
        self.0.ends_with('A')
    }
    fn is_ghost_end(&self) -> bool {
        self.0.ends_with('Z')
    }
}

#[derive(Debug)]
struct NodeOptions<'a>(Node<'a>, Node<'a>);

impl<'a> From<&'a str> for NodeOptions<'a> {
    fn from(value: &'a str) -> Self {
        let (n1, n2) = value
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(',')
            .unwrap();
        Self(Node::from(n1), Node::from(n2))
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::L,
            _ => Self::R,
        }
    }
}


struct NodesPath<'a>{
    end: Node<'a>,
    directions: &'a [Direction]
}


pub fn step(input_content: &str) {

    let lines = input_content.lines().collect::<Vec<&str>>();
    let directions = lines[0]
        .chars()
        .map(Direction::from)
        .collect::<Vec<Direction>>();
    let nodes_map = lines[1..]
        .iter()
        .filter(|&n| !n.is_empty())
        .map(|n: &&str| {
            let (target_node, node_options) = n.split_once('=').unwrap();
            (Node::from(target_node), NodeOptions::from(node_options))
        })
        .collect::<HashMap<Node, NodeOptions>>();

    let mut nodes = nodes_map.keys().filter(|n| n.is_ghost_start()).collect::<Vec<&Node<'_>>>();
    let mut step = 0;
    loop {
        let direction_idx = step % directions.len();
        for (node_idx, node) in nodes.clone().iter().enumerate(){
            nodes[node_idx]= directions[direction_idx].choose_node(&nodes_map[node]);
        }
        if nodes.iter().all(|&n|n.is_ghost_end()) {
            break;
        }
        step += 1;
    }
    dbg!(step);
}



#[bench]
fn bench_challenge_8(bencher: &mut Bencher) {
    step("/workspaces/advent_of_code_2023/inputs/challenge_8.txt")
}
