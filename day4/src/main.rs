use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::fs;

fn part_one(file_path: &str) {
    let lines = file_path.lines();
    let mut graph = Graph::<&str, i32>::new();
    let mut nodes: Vec<NodeIndex> = Vec::new();
    let line_length = 0;
    lines.for_each(|line| {
        let letters = line.split("");
        letters.for_each(|x| {
            if !x.is_empty() {
                let node = graph.add_node(x);
                if let Some(last_node) = nodes.last() {
                    graph.add_edge(*last_node, node, 0);
                }
                nodes.push(node);
            }
        });
    });
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}

fn main() {
    let contents = fs::read_to_string("test.txt").expect("File should be open");
    println!("{contents}");
    part_one(&contents);
}
