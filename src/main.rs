use std::io::{self, Write};
use std::path::Path;
use std::collections::HashMap;
use std::fs;
use serde_json;

mod graph;
use graph::Graph;

fn main() {
    println!("Welcome to Graph Path Finder!");

    let graph_file_path = "/Users/iji/Desktop/Project_real/fastest_route_finder/src/roadNet-CA.txt";
    let mut graph = match Graph::new(Path::new(&graph_file_path)) {
        Ok(graph) => graph,
        Err(err) => {
            eprintln!("Problem reading the file: {}", err);
            std::process::exit(1);
        }
    };

    loop {
        println!("\nChoose an option:");
        println!("1. Find shortest path");
        println!("2. Visualize graph");
        println!("3. Save graph to file");
        println!("4. Load graph from file");
        println!("5. Exit");

        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 5.");
                continue;
            }
        };

        match choice {
            1 => find_shortest_path(&mut graph),
            2 => visualize_graph(&graph),
            3 => save_graph(&graph),
            4 => load_graph(&mut graph),
            5 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 5."),
        }
    }
}

fn find_shortest_path(graph: &mut Graph) {
    let start_node: u32 = loop {
        println!("Enter start node:");
        let input = get_input();
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    };

    let end_node: u32 = loop {
        println!("Enter end node:");
        let input = get_input();
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    };

    if let Some(path) = graph.bfs_shortest_path(start_node, end_node) {
        println!("Shortest path from {} to {}: {:?}", start_node, end_node, path);
    } else {
        println!("No path found from {} to {}", start_node, end_node);
    }
}

fn visualize_graph(graph: &Graph) {
    println!("Graph Visualization:");
    for (node, neighbors) in graph.get_adjacency_list() {
        print!("Node {}: ", node);
        for &neighbor in neighbors {
            print!("{} ", neighbor);
        }
        println!();
    }
}

fn save_graph(graph: &Graph) {
    println!("Enter file path to save the graph:");
    let file_path = get_input();

    match serde_json::to_string(&graph.get_adjacency_list()) {
        Ok(json_str) => {
            if let Err(err) = fs::write(&file_path, json_str) {
                println!("Failed to save graph: {}", err);
            } else {
                println!("Graph saved to file: {}", file_path);
            }
        }
        Err(err) => println!("Failed to serialize graph data: {}", err),
    }
}

fn load_graph(graph: &mut Graph) {
    println!("Enter file path to load the graph:");
    let file_path = get_input();

    match fs::read_to_string(&file_path) {
        Ok(json_str) => {
            match serde_json::from_str::<HashMap<u32, Vec<u32>>>(&json_str) {
                Ok(adj_list) => {
                    graph.set_adjacency_list(adj_list);
                    println!("Graph loaded from file: {}", file_path);
                }
                Err(err) => println!("Failed to deserialize graph data: {}", err),
            }
        }
        Err(err) => println!("Failed to load graph: {}", err),
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    #[test]
fn test_bfs_shortest_path() {
    // Create a sample graph
    let graph = Graph {
        adjacency_list: hashmap![
            1 => vec![2, 3],
            2 => vec![4, 5],
            3 => vec![6],
            4 => vec![],
            5 => vec![6],
            6 => vec![],
        ],
    };

        // Test shortest path from node 1 to node 6
        let start_node = 1;
        let end_node = 6;
        let expected_path = vec![1, 3, 6];
        assert_eq!(graph.bfs_shortest_path(start_node, end_node), Some(expected_path));

        // Test shortest path from node 2 to node 6
        let start_node = 2;
        let end_node = 6;
        let expected_path = vec![2, 5, 6];
        assert_eq!(graph.bfs_shortest_path(start_node, end_node), Some(expected_path));

        // Test when there is no path from node 4 to node 6
        let start_node = 4;
        let end_node = 6;
        assert_eq!(graph.bfs_shortest_path(start_node, end_node), None);
    }
}
