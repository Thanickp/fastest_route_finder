mod graph;
use graph::Graph;
use std::io;
use std::path::Path;

fn main() {
    let graph_file_path = "/Users/iji/Desktop/Project_real/fastest_route_finder/src/roadNet-CA.txt"; 

    let graph = Graph::new(Path::new(&graph_file_path)).unwrap_or_else(|err| {
        eprintln!("Problem reading the file: {}", err);
        std::process::exit(1);
    });

    println!("Enter start node:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let start_node: u32 = input.trim().parse().expect("Invalid input for start node");

    println!("Enter end node:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let end_node: u32 = input.trim().parse().expect("Invalid input for end node");

    if let Some(path) = graph.bfs(start_node, end_node) {
        println!("BFS path from {} to {}: {:?}", start_node, end_node, path);
    } else {
        println!("No path found using BFS from {} to {}", start_node, end_node);
    }

    if let Some(path) = graph.dfs(start_node, end_node) {
        println!("DFS path from {} to {}: {:?}", start_node, end_node, path);
    } else {
        println!("No path found using DFS from {} to {}", start_node, end_node);
    }
}
