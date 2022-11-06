use crate::graph::definitions::*;
use crate::graph::nodes_edges::*;
use crate::graph::edge_list::*;

use rand::distributions::{Distribution, Uniform};

//return directed path
pub fn directed_path(num_nodes: usize) -> EdgeList {
    let mut edges: Vec<(NodeId, DirectedWeightedEdge)> = vec![(0, DirectedWeightedEdge::new()); num_nodes - 1];
    edges.reserve_exact(num_nodes);
    for i in 0..(num_nodes - 1) {
        edges[i].0 = i as NodeId;
        edges[i].1.to = i as NodeId + 1;
        edges[i].1.weight = 1;
    }
    edges.into()
}

//random directed graph, all loops and multiedges are removed after insertion of num_edges edges
pub fn random_graph(num_nodes: usize, num_edges: usize) -> EdgeList {
    let mut edges: Vec<(NodeId, DirectedWeightedEdge)> = vec![(0, DirectedWeightedEdge::new()); num_edges];
    let distr: Uniform<NodeId> = Uniform::from(0..(num_nodes as NodeId));
    let mut rng = rand::thread_rng();
    for _ in 0..num_edges {
        let v = distr.sample(&mut rng);
        let w = distr.sample(&mut rng);
        let weight = 1 + distr.sample(&mut rng) as Weight;
        edges.push((v, DirectedWeightedEdge::from_values(w, weight)));
    }
    let edges = EdgeList::remove_loops_and_multiedges(edges);
    edges.into()
}

