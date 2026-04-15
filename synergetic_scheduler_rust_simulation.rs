// Synergetic Topology-Based Scheduler Simulation with Visualization
// Run with: cargo run
// Requires: cargo add petgraph rand plotters

use petgraph::graph::{Graph, NodeIndex};
use rand::Rng;
use plotters::prelude::*;

#[derive(Clone, Debug)]
struct NodeData {
    resource_demand: f64,
    allocated: f64,
    cpu_time: f64,
}

fn compute_score(node: &NodeData, connectivity: f64) -> f64 {
    let r = node.resource_demand / (node.allocated + 1.0);
    let c = connectivity;
    let l = rand::random::<f64>(); // simulated locality
    let e = rand::random::<f64>(); // simulated emergence
    let d = node.cpu_time;

    1.0 * r + 0.8 * c + 0.6 * l + 0.7 * e - 0.5 * d
}

fn main() {
    let mut graph: Graph<NodeData, f64> = Graph::new();
    let mut rng = rand::thread_rng();

    // Create nodes
    let mut nodes = vec![];
    for _ in 0..20 {
        let n = graph.add_node(NodeData {
            resource_demand: rng.gen_range(1.0..10.0),
            allocated: rng.gen_range(1.0..5.0),
            cpu_time: 0.0,
        });
        nodes.push(n);
    }

    // Create random connections
    for i in 0..nodes.len() {
        for j in i+1..nodes.len() {
            if rng.gen_bool(0.3) {
                graph.add_edge(nodes[i], nodes[j], rng.gen_range(0.5..2.0));
            }
        }
    }

    let mut history = vec![];

    // Simulation loop
    for step in 0..100 {
        let mut best_node = None;
        let mut best_score = f64::MIN;

        for node in graph.node_indices() {
            let data = &graph[node];
            let connectivity: f64 = graph.edges(node).map(|e| *e.weight()).sum();
            let score = compute_score(data, connectivity);

            if score > best_score {
                best_score = score;
                best_node = Some(node);
            }
        }

        // Execute selected node
        if let Some(n) = best_node {
            let node = graph.node_weight_mut(n).unwrap();
            node.cpu_time += 1.0;
            node.allocated += 0.5;
        }

        history.push(best_score);
        println!("Step {}: Best Score = {}", step, best_score);
    }

    // Visualization
    let root = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Scheduler Score Over Time", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..100, 0f64..20f64)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            history.iter().enumerate().map(|(x, y)| (x, *y)),
            &RED,
        ))
        .unwrap();

    println!("Visualization saved to output.png");
}
