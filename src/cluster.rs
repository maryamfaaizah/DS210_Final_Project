use petgraph::{
    graph::{Graph, NodeIndex, UnGraph},
    visit::{EdgeRef, NodeIndexable, Dfs},
    EdgeType,
    Undirected,
};
use std::collections::HashMap;
use crate::graph::Individual;

pub fn component_mapping<N, E, Ty: EdgeType>(graph: &Graph<N, E, Ty>) -> Vec<usize> {
    let mut component = vec![0; graph.node_bound()];
    let mut visited = vec![false; graph.node_bound()];
    let mut dfs = Dfs::new(graph, NodeIndex::new(0));
    let mut component_number = 0;

    for node_index in graph.node_indices() {
        if !visited[node_index.index()] {
            dfs.move_to(node_index);
            while let Some(nx) = dfs.next(graph) {
                if !visited[nx.index()] {
                    visited[nx.index()] = true;
                    component[nx.index()] = component_number;
                }
            }
            component_number += 1;
        }
    }
    component
}

pub fn cluster_analysis(graph: &Graph<Individual, f64, Undirected>) {
    let component_map = component_mapping(graph);
    let num_components = *component_map.iter().max().unwrap_or(&0) + 1;
    let mut components: Vec<Vec<NodeIndex>> = vec![vec![]; num_components];

    for node in graph.node_indices() {
        let component_index = component_map[node.index()];
        components[component_index].push(node);
    }

    for (idx, nodes) in components.iter().enumerate() {
        let cluster_nodes: Vec<_> = nodes.iter().map(|&node_idx| graph[node_idx].clone()).collect();
        println!("Cluster {}: {:?}", idx, cluster_nodes);
    }
}

pub fn detect_communities(graph: &UnGraph<Individual, f64>) -> Vec<Vec<NodeIndex>> {
    let mut communities = Vec::new();
    let mut labels = HashMap::new();
    let mut node_labels = vec![0; graph.node_bound()];
    
    for node in graph.node_indices() {
        labels.insert(node.index(), node.index());
        node_labels[node.index()] = node.index();
    }

    let mut changed = true;
    while changed {
        changed = false;
        let mut new_labels = node_labels.clone();

        for node in graph.node_indices() {
            let mut neighbor_labels = HashMap::new();
            for edge in graph.edges(node) {
                *neighbor_labels.entry(node_labels[edge.target().index()])
                    .or_insert(0) += 1;
            }

            if let Some((most_common_label, _)) = neighbor_labels.into_iter()
                .max_by_key(|&(_, count)| count) {
                if most_common_label != node_labels[node.index()] {
                    new_labels[node.index()] = most_common_label;
                    changed = true;
                }
            }
        }

        node_labels = new_labels;
    }

    let mut community_map = HashMap::new();
    for (node_idx, &label) in node_labels.iter().enumerate() {
        community_map.entry(label).or_insert_with(Vec::new).push(NodeIndex::new(node_idx));
    }

    communities.extend(community_map.values().cloned());
    communities
}


pub fn select_representatives(graph: &UnGraph<Individual, f64>, communities: &[Vec<NodeIndex>], k: usize) -> Vec<NodeIndex> {
    let mut representatives = Vec::new();

    for community in communities {
        let mut community_nodes = community.iter()
            .map(|&node| (node, graph.edges(node).count())) 
            .collect::<Vec<_>>();

        community_nodes.sort_by(|a, b| b.1.cmp(&a.1));
        representatives.extend(community_nodes.into_iter().map(|(node, _)| node).take(k));
    }
    representatives
}

pub fn evaluate_representatives(graph: &UnGraph<Individual, f64>, communities: &[Vec<NodeIndex>], representatives: &[NodeIndex]) {
    for (i, community) in communities.iter().enumerate() {
        let mut avg_bmi = 0.0;
        let mut avg_waist_circumference = 0.0;
        let mut avg_bp_systolic = 0.0;
        let mut avg_bp_diastolic = 0.0;
        let mut avg_blood_glucose_fasting = 0.0;
        let mut avg_total_cholesterol = 0.0;
        let mut avg_triglyceride = 0.0;
        let mut avg_hdl = 0.0;
        let mut avg_ldl = 0.0;
        let mut avg_hba1c = 0.0;

        for &node in community {
            let ind = &graph[node];
            avg_bmi += ind.bmi;
            avg_waist_circumference += ind.waist_circumference;
            avg_bp_systolic += ind.bp_systolic;
            avg_bp_diastolic += ind.bp_diastolic;
            avg_blood_glucose_fasting += ind.blood_glucose_fasting;
            avg_total_cholesterol += ind.total_cholesterol;
            avg_triglyceride += ind.triglyceride;
            avg_hdl += ind.hdl;
            avg_ldl += ind.ldl;
            avg_hba1c += ind.hba1c;
        }

        let count = community.len() as f64;
        avg_bmi /= count;
        avg_waist_circumference /= count;
        avg_bp_systolic /= count;
        avg_bp_diastolic /= count;
        avg_blood_glucose_fasting /= count;
        avg_total_cholesterol /= count;
        avg_triglyceride /= count;
        avg_hdl /= count;
        avg_ldl /= count;
        avg_hba1c /= count;

        for &rep in representatives.iter().filter(|&&n| community.contains(&n)) {
            let rep_ind = &graph[rep];
            println!("\nCommunity {}: Representative Node ID {}:", i, graph.to_index(rep));
            println!("   - BMI: {} (Community Avg: {})", rep_ind.bmi, avg_bmi);
            println!("   - Waist Circumference: {} (Community Avg: {})", rep_ind.waist_circumference, avg_waist_circumference);
            println!("   - Systolic BP: {} (Community Avg: {})", rep_ind.bp_systolic, avg_bp_systolic);
            println!("   - Diastolic BP: {} (Community Avg: {})", rep_ind.bp_diastolic, avg_bp_diastolic);
            println!("   - Blood Glucose Fasting: {} (Community Avg: {})", rep_ind.blood_glucose_fasting, avg_blood_glucose_fasting);
            println!("   - Total Cholesterol: {} (Community Avg: {})", rep_ind.total_cholesterol, avg_total_cholesterol);
            println!("   - Triglyceride: {} (Community Avg: {})", rep_ind.triglyceride, avg_triglyceride);
            println!("   - High-density Lipoproteins: {} (Community Avg: {})", rep_ind.hdl, avg_hdl);
            println!("   - Low-density Lipoproteins: {} (Community Avg: {})", rep_ind.ldl, avg_ldl);
            println!("   - Glycated Hemoglobin: {} (Community Avg: {})", rep_ind.hba1c, avg_hba1c);

        }
    }
}
