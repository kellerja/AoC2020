use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;
use std::ptr;

pub struct UniqueRootsCounter;
pub struct BagCapacityCounter;

pub trait Counter {
    fn count(&self, nodes: &HashMap<String, Node>, root: &str) -> Option<usize>;
}

impl UniqueRootsCounter {
    fn count_unique_roots(&self, nodes: &HashMap<String, Node>, current: &Node, visited: &mut HashSet<String>) -> usize {
        let mut roots_count = 0;
        for edge in &current.parents {
            let parent_name = &edge.other_node;
            if visited.contains(parent_name) {
                continue;
            }
            visited.insert(parent_name.to_owned());
            roots_count += 1 + self.count_unique_roots(nodes, nodes.get(parent_name).unwrap(), visited);
        }
        roots_count
    }
}

impl Counter for UniqueRootsCounter {
    fn count(&self, nodes: &HashMap<String, Node>, root: &str) -> Option<usize> {
        nodes.get(root).and_then(|target| {
            let mut visited = HashSet::new();
            visited.insert(root.to_owned());
            Some(self.count_unique_roots(&nodes, target, &mut visited))
        })
    }
}

impl BagCapacityCounter {
    fn count_bag_capacity(&self, nodes: &HashMap<String, Node>, current: &Node, target: &Node) -> usize {
        let mut bag_count = 0;
        for edge in &current.children {
            let child_name = &edge.other_node;
            let child_node = nodes.get(child_name).unwrap();
            if ptr::eq(child_node, target) {
                panic!("Loop detected")
            }
            let child_bag_count = self.count_bag_capacity(nodes, child_node, target);
            bag_count += edge.cost + child_bag_count * edge.cost;
        }
        bag_count
    }
}

impl Counter for BagCapacityCounter {
    fn count(&self, nodes: &HashMap<String, Node>, root: &str) -> Option<usize> {
        nodes.get(root).and_then(|target| Some(self.count_bag_capacity(&nodes, target, target)))
    }
}

#[derive(Debug)]
pub struct Node {
    parents: Vec<Edge>,
    children: Vec<Edge>
}

impl Node {
    fn new() -> Node {
        Node {
            parents: Vec::new(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Edge {
    other_node: String,
    cost: usize
}

impl Edge {
    fn new(other_node: &str, cost: usize) -> Edge {
        Edge {
            other_node: other_node.to_owned(),
            cost
        }
    }
}



pub fn solve(input: &File, counter: &impl Counter) -> Option<usize> {
    const TARGET_NAME: &str = "shiny gold";
    let nodes = parse_input(input);
    counter.count(&nodes, TARGET_NAME)
}

fn parse_input(input: &File) -> HashMap<String, Node> {
    let child_pattern = Regex::new(r"(?P<cost>[[:digit:]]+) (?P<node>.+) bags?").unwrap();
    let line_pattern = Regex::new(r"^(?P<node>.+) bags contain (?:no other bags|(?P<children>.*)+)\.$").unwrap();

    let mut nodes = HashMap::new();
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let cap = line_pattern.captures(&line).unwrap();
        let node_name = &cap["node"];
        nodes.entry(node_name.to_owned()).or_insert(Node::new());
        if let Some(children_group) = cap.name("children") {
            let children: Vec<&str> = children_group.as_str().split(", ").collect();
            for child in children {
                let child = child_pattern.captures(child).unwrap();
                let child_node_name = &child["node"];
                let cost = child["cost"].parse().unwrap();
                let child_node = nodes.entry(child_node_name.to_owned()).or_insert(Node::new());
                child_node.parents.push(Edge::new(node_name, cost));
                nodes.entry(node_name.to_owned()).and_modify(|node| node.children.push(Edge::new(child_node_name, cost)));
            }
        }
    };
    nodes
}
