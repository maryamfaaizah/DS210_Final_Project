use petgraph::graph::UnGraph;
// putting different weights on metrics based on significance
const HBA1C_WEIGHT: f64 = 1.5;
const SYSTOLIC_BP_WEIGHT: f64 = 1.2;
const DIASTOLIC_BP_WEIGHT: f64 = 1.0;
const GLUCOSE_WEIGHT: f64 = 1.3;
const LDL_WEIGHT: f64 = 1.2;
const HDL_WEIGHT: f64 = 1.1;
const TRIGLYCERIDE_WEIGHT: f64 = 1.0;
const TOTAL_CHOLESTEROL_WEIGHT: f64 = 1.1;
const BMI_WEIGHT: f64 = 0.8;
const WAIST_CIRCUMFERENCE_WEIGHT: f64 = 1.1;

#[derive(Clone, Debug)]
pub struct Individual {
    pub bmi: f64,
    pub waist_circumference: f64,
    pub bp_systolic: f64,
    pub bp_diastolic: f64,
    pub blood_glucose_fasting: f64,
    pub total_cholesterol: f64,
    pub triglyceride: f64,
    pub hdl: f64,
    pub ldl: f64,
    pub hba1c: f64,
}

pub fn similarity(individual1: &Individual, individual2: &Individual) -> f64 {
    let mut sum_squares = 0.0;
    sum_squares += ((individual1.bmi - individual2.bmi).powi(2)) * BMI_WEIGHT;
    sum_squares += ((individual1.waist_circumference - individual2.waist_circumference).powi(2)) * WAIST_CIRCUMFERENCE_WEIGHT;
    sum_squares += ((individual1.bp_systolic - individual2.bp_systolic).powi(2)) * SYSTOLIC_BP_WEIGHT;
    sum_squares += ((individual1.bp_diastolic - individual2.bp_diastolic).powi(2)) * DIASTOLIC_BP_WEIGHT;
    sum_squares += ((individual1.blood_glucose_fasting - individual2.blood_glucose_fasting).powi(2)) * GLUCOSE_WEIGHT;
    sum_squares += ((individual1.total_cholesterol - individual2.total_cholesterol).powi(2)) * TOTAL_CHOLESTEROL_WEIGHT;
    sum_squares += ((individual1.triglyceride - individual2.triglyceride).powi(2)) * TRIGLYCERIDE_WEIGHT;
    sum_squares += ((individual1.hdl - individual2.hdl).powi(2)) * HDL_WEIGHT;
    sum_squares += ((individual1.ldl - individual2.ldl).powi(2)) * LDL_WEIGHT;
    sum_squares += ((individual1.hba1c - individual2.hba1c).powi(2)) * HBA1C_WEIGHT;

    let euclidean_distance = sum_squares.sqrt();
    1.0 / (1.0 + euclidean_distance)
}

pub fn build_graph(individuals: &[Individual], threshold: f64) -> UnGraph<Individual, f64> {
    let mut graph = UnGraph::<Individual, f64>::new_undirected();
    let mut node_indices = Vec::new();

    for individual in individuals {
        let node_idx = graph.add_node(individual.clone());
        node_indices.push(node_idx);
    }

    for i in 0..individuals.len() {
        for j in i + 1..individuals.len() {
            let similar = similarity(&individuals[i], &individuals[j]);
            if similar > threshold {
                graph.add_edge(node_indices[i], node_indices[j], similar);
            }
        }
    }
    graph
}

pub fn vertex_degree(graph: &UnGraph<Individual, f64>) {
    for node in graph.node_indices() {
        let degree = graph.edges(node).count();
        println!("Node {:?} has a degree of {}", graph[node], degree);
    }
}


