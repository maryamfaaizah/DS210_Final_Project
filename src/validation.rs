use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::visit::{IntoNodeIdentifiers, EdgeRef};
use std::collections::{HashSet, HashMap};

// test 1
pub fn test_graph_integrity<N, E>(graph: &UnGraph<N, E>) -> bool {
    let node_count = graph.node_count();
    let nodes: HashSet<_> = graph.node_identifiers().collect();
    nodes.len() == node_count
}

fn avg_dissimilarity<N, E>(graph: &UnGraph<N, E>, node: NodeIndex, cluster_nodes: &[NodeIndex]) -> f64
where
    N: Clone,
    E: Clone + Into<f64>,
{
    cluster_nodes.iter()
        .filter(|&&n| n != node)
        .map(|&n| graph.find_edge(node, n)
            .and_then(|edge_index| graph.edge_weight(edge_index)) // Get the edge weight using the edge index
            .map_or(f64::INFINITY, |weight| (*weight).clone().into())) // Convert weight to f64 or return INFINITY if no edge exists
        .sum::<f64>() / cluster_nodes.len() as f64
}

// Test 2
pub fn calculate_silhouette_scores<N, E>(graph: &UnGraph<N, E>, clusters: &HashMap<u32, Vec<NodeIndex>>) -> HashMap<NodeIndex, f64>
where
    N: Clone,
    E: Clone + Into<f64>,
{
    let mut silhouette_scores = HashMap::new();

    for (&cluster_id, nodes) in clusters.iter() {
        for &node in nodes {
            let a = avg_dissimilarity(graph, node, nodes);

            let b = clusters.iter()
                .filter(|(&other_cluster_id, _)| other_cluster_id != cluster_id)
                .map(|(_, other_nodes)| avg_dissimilarity(graph, node, other_nodes))
                .min_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or(f64::INFINITY);

            let s = if a < b {
                1.0 - (a / b)
            } else if a > b {
                (b / a) - 1.0
            } else {
                0.0
            };

            silhouette_scores.insert(node, s);
        }
    }
    silhouette_scores
}
