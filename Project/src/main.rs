mod graph;

use graph::Graph;
use std::collections::{HashMap, VecDeque};
use csv::ReaderBuilder;
use std::error::Error;
//use std::fs::File;
//use std::io::Write;

// Function to load data from the CSV and build a graph
fn load_data(file_path: &str) -> Result<Graph, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let mut graph = Graph::new();

    for result in rdr.records() {
        let record = result?;
        let source = record.get(3).unwrap_or("").trim().to_lowercase();
        let destination = record.get(7).unwrap_or("").trim().to_lowercase();

        if source.is_empty() || destination.is_empty() {
            eprintln!("Skipping invalid row: {:?}", record);
            continue;
        }

        graph.add_edge(source, destination);
    }

    Ok(graph)
}

// Function to calculate degrees of separation using BFS
fn degrees_of_separation(graph: &Graph, start: &str, end: &str) -> Option<usize> {
    let adj_list = graph.get_adj_list();

    if !adj_list.contains_key(start) || !adj_list.contains_key(end) {
        println!("City not found in graph: {} or {}", start, end);
        return None;
    }

    let mut visited: HashMap<String, bool> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start.to_string(), 0));
    visited.insert(start.to_string(), true);

    while let Some((current, degree)) = queue.pop_front() {
        if current == end {
            return Some(degree);
        }

        if let Some(neighbors) = adj_list.get(&current) {
            for neighbor in neighbors {
                if !visited.contains_key(neighbor) {
                    queue.push_back((neighbor.clone(), degree + 1));
                    visited.insert(neighbor.clone(), true);
                }
            }
        }
    }

    None
}

fn calculate_mae(predictions: &[f64], actuals: &[f64]) -> f64 {
    if predictions.len() != actuals.len() {
        panic!("Predictions and actuals must have the same length");
    }

    predictions
        .iter()
        .zip(actuals.iter())
        .map(|(pred, actual)| (pred - actual).abs())
        .sum::<f64>()
        / predictions.len() as f64
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "flight_data.csv";
    let graph = load_data(file_path)?;

    println!("Enter the start city:");
    let mut start = String::new();
    std::io::stdin().read_line(&mut start)?;
    let start = start.trim().to_lowercase();

    println!("Enter the destination city:");
    let mut end = String::new();
    std::io::stdin().read_line(&mut end)?;
    let end = end.trim().to_lowercase();

    let predictions = vec![100.0, 200.0, 300.0, 400.0];
    let actuals = vec![110.0, 190.0, 310.0, 390.0];
    let mae = calculate_mae(&predictions, &actuals);
    println!("Mean Absolute Error (MAE): {:.2}", mae);

    match degrees_of_separation(&graph, &start, &end) {
        Some(degrees) => println!("Degrees of separation between {} and {}: {}", start, end, degrees),
        None => println!("No connection between {} and {}", start, end),
    }

    println!("\nCentrality Metrics:");

    // Degree Centrality
    let degree_centrality = graph.degree_centrality();
    println!("Degree Centrality: {:?}", degree_centrality);

    // Closeness Centrality
    if let Some(closeness) = graph.closeness_centrality(&start) {
        println!("Closeness Centrality for {}: {:.4}", start, closeness);
    } else {
        println!("Closeness Centrality: {} is disconnected.", start);
    }

    // Betweenness Centrality
    let betweenness_centrality = graph.betweenness_centrality();
    println!("Betweenness Centrality: {:?}", betweenness_centrality);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph::Graph;

    #[test]
    fn test_degrees_of_separation() {
        let mut graph = Graph::new();
        graph.add_edge("Mumbai".to_string(), "Hyderabad".to_string());
        graph.add_edge("Mumbai".to_string(), "Delhi".to_string());
        graph.add_edge("Hyderabad".to_string(), "Chennai".to_string());

        assert_eq!(
            degrees_of_separation(&graph, "Mumbai", "Hyderabad"),
            Some(1)
        );
        assert_eq!(degrees_of_separation(&graph, "Mumbai", "Chennai"), Some(2));
        assert_eq!(degrees_of_separation(&graph, "Mumbai", "Delhi"), Some(1));
        assert_eq!(degrees_of_separation(&graph, "Delhi", "Chennai"), Some(3));
        assert_eq!(
            degrees_of_separation(&graph, "Mumbai", "NonExistentCity"),
            None
        );
    }

    #[test]
    fn test_no_connection() {
        let mut graph = Graph::new();
        graph.add_edge("CityA".to_string(), "CityB".to_string());
        graph.add_edge("CityC".to_string(), "CityD".to_string());

        assert_eq!(degrees_of_separation(&graph, "CityA", "CityD"), None);
    }

    #[test]
    fn test_single_node_graph() {
        let mut graph = Graph::new();
        graph.add_edge("CityA".to_string(), "CityA".to_string());

        assert_eq!(degrees_of_separation(&graph, "CityA", "CityA"), Some(0));
        assert_eq!(degrees_of_separation(&graph, "CityA", "CityB"), None);
    }
}
