use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::algo::{dijkstra, astar};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::Path;
use serde_json;

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
        let mut writer = io::BufWriter::new(file);
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