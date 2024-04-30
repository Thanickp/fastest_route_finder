use std::collections::{HashMap, VecDeque, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct Graph {
    adjacency_list: HashMap<u32, Vec<u32>>,
}

impl Graph {
    pub fn new(file_path: &Path) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut adjacency_list = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            if !line.starts_with('#') {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 2 {
                    let from: u32 = parts[0].parse().expect("Invalid node id");
                    let to: u32 = parts[1].parse().expect("Invalid node id");
                    adjacency_list.entry(from).or_insert(vec![]).push(to);
                }
            }
        }
        Ok(Graph { adjacency_list })
    }

    pub fn bfs(&self, start: u32, goal: u32) -> Option<Vec<u32>> {
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

    pub fn dfs(&self, start: u32, goal: u32) -> Option<Vec<u32>> {
        let mut stack = vec![start];
        let mut visited = HashSet::new();
        let mut path = HashMap::new();

        visited.insert(start);
        path.insert(start, None);

        while let Some(current) = stack.pop() {
            if current == goal {
                return Some(self.construct_path(start, goal, &path));
            }

            if let Some(neighbors) = self.adjacency_list.get(&current) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        stack.push(neighbor);
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
}
