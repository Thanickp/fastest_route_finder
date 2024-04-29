use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::algo::{dijkstra, astar};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write, stdin};
use std::path::{Path, PathBuf};
use serde_json;
use csv;
use std::env;


#[derive(Debug, Serialize, Deserialize)]
pub struct Road {
    source: usize,
    target: usize,
    distance: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub delimiter: u8,
    pub data_path: String,
}

pub struct RoadNetwork {
    pub graph: UnGraph<usize, f32>,
    pub index_map: HashMap<usize, NodeIndex>,
}

impl RoadNetwork {
    pub fn new() -> Self {
        Self {
            graph: UnGraph::new_undirected(),
            index_map: HashMap::new(),
        }
    }

    pub fn add_road(&mut self, road: Road) {
        let source_index = *self.index_map.entry(road.source).or_insert_with(|| self.graph.add_node(road.source));
        let target_index = *self.index_map.entry(road.target).or_insert_with(|| self.graph.add_node(road.target));
        self.graph.add_edge(source_index, target_index, road.distance);
    }

    pub fn load_roads_from_file<P: AsRef<Path>>(&mut self, path: P, delimiter: u8) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut csv_reader = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .from_reader(reader);

        for result in csv_reader.deserialize() {
            let road: Road = result?;
            self.add_road(road);
        }
        Ok(())
    }

    pub fn save_config<P: AsRef<Path>>(&self, path: P, config: &Config) -> io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &config)?;
        writer.flush()
    }

    pub fn load_config<P: AsRef<Path>>(path: P) -> io::Result<Config> { 
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn shortest_path_dijkstra(&self, start: usize, end: usize) -> Option<f32> {
        let start_index = self.index_map.get(&start)?;
        let end_index = self.index_map.get(&end)?;
        let result = dijkstra(&self.graph, *start_index, Some(*end_index), |e| *e.weight());
        result.get(end_index).cloned()
    }
    
    pub fn shortest_path_astar(&self, start: usize, end: usize) -> Option<f32> {
        let start_index = self.index_map.get(&start)?;
        let end_index = self.index_map.get(&end)?;
        let result = astar(
            &self.graph,
            *start_index,
            |finish| finish == *end_index,
            |e| *e.weight(),
            |_| 0.0 
        );
        result.map(|(cost, _)| cost)
    }
}

fn main() -> io::Result<()> {
    println!("Welcome to the Road Network Analyzer!");
    println!("Current directory: {:?}", env::current_dir()?);

    let config_path = PathBuf::from("config.json");
    let config = RoadNetwork::load_config(&config_path).unwrap_or_else(|_| {
        println!("No configuration found. Using default settings.");
        Config {
            delimiter: b',',
            data_path: "Users/iji/Desktop/Project_real/fastest_route_finder/Ca_road.csv".to_string(),
        }
    });

    let mut network = RoadNetwork::new();
    println!("Loading roads from file: {}", config.data_path);
    let data_path = PathBuf::from(&config.data_path);
    if !data_path.exists() {
        println!("Error: Data file not found at {}", config.data_path);
        return Ok(());
    }
    network.load_roads_from_file(&data_path, config.delimiter)?;

    println!("Roads loaded successfully. Please enter the start and end node IDs:");
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let nodes: Vec<usize> = input.trim().split_whitespace()
                                 .filter_map(|s| s.parse().ok())
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
