use std::{collections::HashMap, env, fs};

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
    fn is_end<'a>(&'a self) -> bool {
        self == &Node::end_node()
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

pub fn run(){
    let input_file_path = env::args().last().unwrap();
    let input_content = fs::read_to_string(input_file_path).unwrap();
    let (directions, nodes) = input_content.split_once('\n').unwrap();
    let directions = directions
        .chars()
        .map(Direction::from)
        .collect::<Vec<Direction>>();
    let nodes_map = nodes
        .split('\n')
        .filter(|&n| !n.is_empty())
        .map(|n| {
            let (target_node, node_options) = n.split_once('=').unwrap();
            (Node::from(target_node), NodeOptions::from(node_options))
        })
        .collect::<HashMap<Node, NodeOptions>>();
    let mut step = 0;
    let mut node = directions[step].choose_node(&nodes_map[&Node::start()]);
    loop {
        node = directions[step % directions.len()].choose_node(&nodes_map[node]);
        if node.is_end() {
            break;
        }
        step += 1;
    }
    dbg!(step);

}