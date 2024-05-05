mod clean;
mod graph;
mod cluster;
mod validation;

use clean::norm_column;
use graph::{build_graph, Individual, vertex_degree};
use cluster::{cluster_analysis, detect_communities, select_representatives, evaluate_representatives};
use validation::{test_graph_integrity, calculate_silhouette_scores}; 
use std::collections::HashMap;
use std::error::Error;
use csv::WriterBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let read_path = "./dataFL.csv";
    let write_path = "./normalized_data_FL.csv";

    let indices = vec![2, 3, 4, 5, 6, 8, 9, 10, 11, 12];
    let threshold = 3.0; 

    let normalized_data = norm_column(&read_path, &indices, threshold)?;

    let mut wtr = WriterBuilder::new().from_path(write_path)?;
    wtr.write_record(&["bmi", "waist_circumference", "blood_pressure_systolic", "blood_pressure_diastolic", "blood_glucose_fasting", "total_cholesterol", "triglyceride", "hdl", "ldl", "hba1c"])?;
    for row in &normalized_data {
        wtr.serialize(row)?;
    }
    wtr.flush()?;
    println!("Data written to {}", write_path);

    let individuals: Vec<Individual> = normalized_data.iter().map(|row| Individual {
        bmi: row[0],
        waist_circumference: row[1],
        bp_systolic: row[2],
        bp_diastolic: row[3],
        blood_glucose_fasting: row[4],
        total_cholesterol: row[5],
        triglyceride: row[6],
        hdl: row[7],
        ldl: row[8],
        hba1c: row[9],
    }).collect();
    let graph = build_graph(&individuals, threshold);

    let integrity = test_graph_integrity(&graph);
    println!("Graph Integrity: {}", integrity);

    let communities = detect_communities(&graph);
    println!("Communities detected: {}", communities.len());

    let community_map: HashMap<u32, Vec<_>> = communities.iter().enumerate()
        .map(|(i, community)| (i as u32, community.clone()))
        .collect();

    let representatives = select_representatives(&graph, &communities, 3); // Example: Select 3 representatives per community

    evaluate_representatives(&graph, &communities, &representatives);

    let silhouette_scores = calculate_silhouette_scores(&graph, &community_map);
    let average_score = silhouette_scores.values().sum::<f64>() / silhouette_scores.len() as f64;
    println!("Average Silhouette Score: {}", average_score);

    vertex_degree(&graph);

    cluster_analysis(&graph);

    println!("Done.");
    Ok(())
}
