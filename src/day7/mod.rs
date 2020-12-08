use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug)]
struct Node {
    parents: Vec<Edge>
}

impl Node {
    fn new() -> Node {
        Node {
            parents: Vec::new()
        }
    }
}

#[derive(Debug)]
struct Edge {
    parent: String,
    cost: usize
}

impl Edge {
    fn new(parent: &str, cost: usize) -> Edge {
        Edge {
            parent: parent.to_owned(),
            cost
        }
    }
}

fn count_unique_roots(nodes: &HashMap<String, Node>, current: Option<&Node>, visited: &mut HashSet<String>) -> usize {
    let parents = match current {
        Some(node) => &node.parents,
        None => return 0
    };
    if parents.is_empty() {
        return 0;
    }
    let mut roots_count = 0;
    for edge in parents {
        let parent_name = &edge.parent;
        if visited.contains(parent_name) {
            continue;
        }
        visited.insert(parent_name.to_owned());
        roots_count += 1 + count_unique_roots(nodes, nodes.get(parent_name), visited);
    }
    roots_count
}

pub fn solve(input: &File) -> Option<usize> {
    let nodes = parse_input(input);
    let target = nodes.get("shiny gold");
    if target.is_none() {
        None
    } else {
        let mut visited = HashSet::new();
        Some(count_unique_roots(&nodes, target, &mut visited))
    }
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
                child_node.parents.push(Edge::new(node_name, cost))
            }
        }
    };
    nodes
}
