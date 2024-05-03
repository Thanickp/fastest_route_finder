use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

#[derive(Debug)]
pub struct Graph {
    adjacency_list: HashMap<u32, Vec<u32>>,
}

impl Graph {
    pub fn new(file_path: &Path) -> Result<Self, String> {
        let contents = match std::fs::read_to_string(file_path) {
            Ok(contents) => contents,
            Err(err) => return Err(format!("Failed to read file: {}", err)),
        };

        let mut adjacency_list = HashMap::new();
        for line in contents.lines() {
            if !line.starts_with('#') {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 2 {
                    let from: u32 = match parts[0].parse() {
                        Ok(num) => num,
                        Err(_) => return Err("Invalid node id".to_string()),
                    };
                    let to: u32 = match parts[1].parse() {
                        Ok(num) => num,
                        Err(_) => return Err("Invalid node id".to_string()),
                    };
                    adjacency_list.entry(from).or_insert(vec![]).push(to);
                }
            }
        }

        Ok(Graph { adjacency_list })
    }

    pub fn bfs_shortest_path(&self, start: u32, goal: u32) -> Option<Vec<u32>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut path = HashMap::new();

        visited.insert(start);
        queue.push_back(start);
        path.insert(start, None);

        while let Some(current) = queue.pop_front() {
            if current == goal {
                return Some(self.construct_path(start, goal, &path));
            }

            if let Some(neighbors) = self.adjacency_list.get(&current) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                        path.insert(neighbor, Some(current));
                    }
                }
            }
        }
        None
    }

    fn construct_path(&self, start: u32, goal: u32, predecessors: &HashMap<u32, Option<u32>>) -> Vec<u32> {
        let mut path = Vec::new();
        let mut current = goal;
        while let Some(&prev) = predecessors.get(&current) {
            path.push(current);
            current = prev.expect("Predecessor not found, which should be impossible in this context");
            if current == start {
                path.push(start);
                break;
            }
        }
        path.reverse();
        path
    }

    pub fn get_adjacency_list(&self) -> &HashMap<u32, Vec<u32>> {
        &self.adjacency_list
    }

    pub fn set_adjacency_list(&mut self, adjacency_list: HashMap<u32, Vec<u32>>) {
        self.adjacency_list = adjacency_list;
    }
}
