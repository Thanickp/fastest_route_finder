use fastest_route_finder::{RoadNetwork, Config};
use std::io::{self};
use std::path::Path;

fn main() -> io::Result<()> {
    println!("Welcome to the Road Network Analyzer!");
    
    let config_path = "config.json";
    let config = match RoadNetwork::load_config(config_path) {
        Ok(cfg) => cfg,
        Err(_) => {
            println!("No configuration found. Using default settings.");
            Config {
                delimiter: b',',
                data_path: "Users/iji/Desktop/Spring2024/DS210/Project/fastest_route_finder/Ca_road.csv".to_string(),
            }
        }
    };

    let mut network = RoadNetwork::new();
    println!("Loading roads from file: {}", config.data_path);
    let data_path = Path::new(&config.data_path);
    if !data_path.exists() {
        println!("Error: Data file not found at {}", config.data_path);
        return Ok(());
    }
    network.load_roads_from_file(data_path, config.delimiter)?;

    println!("Roads loaded successfully. Please enter the start and end node IDs:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let nodes: Vec<usize> = input.trim().split_whitespace()
                                 .map(|s| s.parse().expect("Please enter valid numbers"))
                                 .collect();

    if nodes.len() < 2 {
        println!("Error: You must enter two numbers for start and end nodes.");
        return Ok(());
    }

    let shortest_distance_dijkstra = network.shortest_path_dijkstra(nodes[0], nodes[1]);
    let shortest_distance_astar = network.shortest_path_astar(nodes[0], nodes[1]);

    println!("Shortest distance (Dijkstra) from {} to {}: {:?}", nodes[0], nodes[1], shortest_distance_dijkstra);
    println!("Shortest distance (A*) from {} to {}: {:?}", nodes[0], nodes[1], shortest_distance_astar);

    Ok(())
}
