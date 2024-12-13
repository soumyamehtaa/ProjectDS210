use std::collections::{HashMap, HashSet, VecDeque};

// Define the Graph structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
    adj_list: HashMap<String, Vec<String>>, // Adjacency list representation
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }
    pub fn get_adj_list(&self) -> &HashMap<String, Vec<String>> {
        &self.adj_list
    }
    // Add an edge between two nodes (undirected)
    pub fn add_edge(&mut self, u: String, v: String) {
        self.adj_list.entry(u.clone()).or_insert_with(Vec::new).push(v.clone());
        self.adj_list.entry(v).or_insert_with(Vec::new).push(u);
    }

    // Degree centrality
    pub fn degree_centrality(&self) -> HashMap<String, usize> {
        let mut centrality = HashMap::new();
        for (node, neighbors) in &self.adj_list {
            centrality.insert(node.clone(), neighbors.len());
        }
        centrality
    }

    // Closeness centrality
    pub fn closeness_centrality(&self, start: &str) -> Option<f64> {
        let mut total_distance = 0;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start.to_string(), 0));
        visited.insert(start.to_string());

        while let Some((node, dist)) = queue.pop_front() {
            total_distance += dist;

            if let Some(neighbors) = self.adj_list.get(&node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back((neighbor.clone(), dist + 1));
                    }
                }
            }
        }

        if visited.len() == self.adj_list.len() {
            Some(1.0 / total_distance as f64)
        } else {
            None
        }
    }

    // Betweenness centrality
    pub fn betweenness_centrality(&self) -> HashMap<String, f64> {
        let mut centrality = HashMap::new();
        let nodes: Vec<String> = self.adj_list.keys().cloned().collect();
        for start in &nodes {
            for end in &nodes {
                if start != end {
                    let paths = self.shortest_paths(start, end);
                    for path in paths {
                        for node in path {
                            *centrality.entry(node).or_insert(0.0) += 1.0;
                        }
                    }
                }
            }
        }

        centrality
    }

    // Find all shortest paths
    pub fn shortest_paths(&self, start: &str, end: &str) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent_map: HashMap<String, Vec<String>> = HashMap::new();
        visited.insert(start.to_string());
        queue.push_back(start.to_string());

        while let Some(node) = queue.pop_front() {
            if node == end {
                let mut path = Vec::new();
                let mut current = end.to_string();
                path.push(current.clone());
                while let Some(parents) = parent_map.get(&current) {
                    for parent in parents {
                        path.push(parent.clone());
                        current = parent.clone();
                    }
                }
                path.reverse();
                paths.push(path);
                continue;
            }

            if let Some(neighbors) = self.adj_list.get(&node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back(neighbor.clone());
                        parent_map.entry(neighbor.clone()).or_insert(Vec::new()).push(node.clone());
                    }
                }
            }
        }

        paths
    }
}
