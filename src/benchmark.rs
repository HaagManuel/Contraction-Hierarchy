
use std::time::Duration;
use std::time::Instant;

use rand::distributions::{Distribution, Uniform};

use crate::ch::ch_graph::CHGraph;
use crate::ch::contraction::BottomUpConfig;
use crate::ch::contraction::ContractionConfig;
use crate::dijkstra::*;
use crate::graph::adjacency_array::AdjacencyArray;
use crate::graph::adjacency_list::AdjacencyList;
use crate::graph::definitions::*;
use crate::graph::nodes_edges::*;
use crate::graph::edge_list::*;

fn measure_time<F: FnMut()>(mut f: F) -> Duration {
    let now: Instant = Instant::now();
    f();
    let elapsed: Duration = now.elapsed();  
    elapsed
}
//distances 2^1, ..., 2^k
fn dijkstra_rank_targets<G>(dij: &mut Dijkstra<DirectedWeightedEdge, G>, start: NodeId) -> Vec<(NodeId, Distance)> 
where 
G: Graph + IncidentEdges<DirectedWeightedEdge> {
    let distances = dij.one_to_all(start);
    let mut targets: Vec<(NodeId, Distance)> = Vec::new();
    let mut ranks: Vec<(Distance, usize)> = std::iter::zip(distances, 0..).collect();
    let mut pow2: Weight = 1;
    ranks.sort();
    for (d, i) in ranks.iter().filter(|(d, _)| *d < crate::dijkstra::INFINITY) {
        if *d >= pow2 {
            targets.push((*i as NodeId, *d));
            while *d >= pow2 {pow2 *= 2;}
        }
    }
    targets
}
// [start [targets]]
fn compute_start_targets(num_starts: usize, edge_list: EdgeList) ->  Vec<(NodeId, Vec<(NodeId, Distance)>)> {
    let distr: Uniform<NodeId> = Uniform::from(0..edge_list.num_nodes() as NodeId);
    let array: AdjacencyArray = edge_list.into();
    let mut data: DijkstraData = DijkstraData::new(array.num_nodes());
    let mut dij: Dijkstra<DirectedWeightedEdge, AdjacencyArray> = Dijkstra::new(&array, &mut data);
    let rng = rand::thread_rng();
    let starts: Vec<NodeId> = distr.sample_iter(rng)
                                   .take(num_starts)
                                   .collect();
    let start_targets: Vec<(NodeId, Vec<(NodeId, Distance)>)> = starts.iter()
                                                                      .map(|&s| (s, dijkstra_rank_targets(&mut dij, s)))
                                                                      .collect();
    return start_targets;
}

pub fn benchmark_array_vs_list(edge_list: EdgeList) {
    let mut dd1: DijkstraData = DijkstraData::new(edge_list.num_nodes());
    let mut dd2: DijkstraData = DijkstraData::new(edge_list.num_nodes());

    let array: AdjacencyArray = edge_list.clone().into();
    let list: AdjacencyList = edge_list.into();
    
    let mut dij = Dijkstra::new(&list, &mut dd2); 
    let mut dij2 = Dijkstra::new(&array, &mut dd1);

    let starts = vec![1, 100, 1000];
    
    println!("one to all - adjacency list");
    for s in &starts {
        println!("{:?}", measure_time(|| {dij.one_to_all(*s);}));
    }
    println!("");

    println!("one to all - adjacency array");
    for s in &starts {
        println!("{:?}", measure_time(|| {dij2.one_to_all(*s);}));
    }
println!("");

}

pub fn ch_from_ordering(edge_list: EdgeList, ordering: Vec<NodeId>) {
    println!("CH from ordering");
    let mut data1: DijkstraData = DijkstraData::new(edge_list.num_nodes());
    let mut data2: DijkstraData = DijkstraData::new(edge_list.num_nodes());
    let now: Instant = Instant::now();
    let _ch: CHGraph = CHGraph::from_ordering(edge_list, ordering, &mut data1, &mut data2, ContractionConfig::default());
    let elapsed: Duration = now.elapsed();    
    println!("{:?}", elapsed);
}

pub fn ch_bottom_up(edge_list: EdgeList) {
    println!("CH bottom up");
    let mut data1: DijkstraData = DijkstraData::new(edge_list.num_nodes());
    let mut data2: DijkstraData = DijkstraData::new(edge_list.num_nodes());
    let now: Instant = Instant::now();
    let _ch: (_, _) = CHGraph::bottom_up_construction(edge_list, &mut data1, &mut data2, ContractionConfig::default(), BottomUpConfig::default());
    let elapsed: Duration = now.elapsed();    
    println!("{:?}", elapsed);
}

pub fn one_to_one_random<T: OneToOne>(mut runner: T, edge_list: EdgeList, num_start: usize) -> Vec<(NodeId, NodeId, Distance, Duration)> {
    let start_targets = compute_start_targets(num_start, edge_list);
    let mut results: Vec<(NodeId, NodeId, Distance, Duration)> = Vec::new();
    for (s, targets) in start_targets {
        for (t, _d) in targets {
            let now: Instant = Instant::now();
            let _d2: Distance = runner.one_to_one(s, t);
            let elapsed: Duration = now.elapsed();    
            assert_eq!(_d, _d2);
            results.push((s, t, _d, elapsed));
        }
    }
    return results;
}

fn int_log2(x: Distance) -> usize {
    return ((x as f64).log2().floor()) as usize
}

//in ms
fn mean_and_std(v: &Vec<Duration>) -> (f64, f64) {
    let avg: f64 = v.iter().map(|&d| d.as_secs_f64()).sum::<f64>() / (v.len() as f64);
    let st: f64 = (v.iter().map(|&d| (d.as_secs_f64() - avg).powi(2)).sum::<f64>() / (v.len() as f64)).sqrt();
    return (avg * 1000f64, st * 1000f64);
}

//print avg and std for buckets 2^x
pub fn eval_result(results: &Vec<(NodeId, NodeId, Distance, Duration)>) {
    let max_exp: usize = results.iter().map(|&x|  int_log2(x.2)).max().unwrap();
    let mut buckets: Vec<Vec<Duration>> = vec![Vec::new(); max_exp + 1];
    for (_, _, d, time) in results {
        buckets[int_log2(*d)].push(*time);
    }
    let statistics: Vec<(f64, f64)> = buckets.iter().map(|v| mean_and_std(&v)).collect();
    for (i, (mean, st)) in statistics.iter().enumerate().filter(|(_, (m, _) )| *m > 0f64) {
        println!("2^{i}: {mean:.4} [ms] +/- {st:.4} [ms]");
    }
}