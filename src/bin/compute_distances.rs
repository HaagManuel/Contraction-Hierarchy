use std::path::Path;

use stud_rust_base::ch::ch_graph::CHGraphRunner;
use stud_rust_base::ch::contraction::{BottomUpConfig, ContractionConfig};
use stud_rust_base::dijkstra::*;
use stud_rust_base::graph::{definitions::*, edge_list::{*}, nodes_edges::*, adjacency_array::AdjacencyArray};

use stud_rust_base::io::*;
use stud_rust_base::time::report_time;
use stud_rust_base::logging::report_progress;

use clap::Parser;

fn read_source_target<P: AsRef<Path>>(path: P) -> (Vec<NodeId>, Vec<NodeId>) {
    let dir = path.as_ref();
    let source: Vec<NodeId> = Vec::<NodeId>::load_from(dir.join("source")).unwrap();
    let target: Vec<NodeId> = Vec::<NodeId>::load_from(dir.join("target")).unwrap();
    return (source, target);
}

fn write_result<P: AsRef<Path>>(path_out: P, result: Vec<Distance>) {
    result.write_to(&path_out).unwrap();
}

fn compute_distances<T: OneToOne>(mut algo: T, source: &Vec<NodeId>, target: &Vec<NodeId>, path_out: &String) {
    let mut result: Vec<Distance> = vec![0; source.len()];
    report_time("compute distances", || {
        for (i, (s,t)) in std::iter::zip(source.iter(), target.iter()).enumerate() {
            report_progress(i, source.len(), "", 10);
            result[i] = algo.one_to_one(*s, *t);
        }
    });

    report_time("Write Results", || {
        eprintln!("{:?}", path_out);
        write_result(path_out, result);
    });
}

fn report_graph(edge_list: &EdgeList) {
    eprintln!("--> N = {}, M = {}", edge_list.num_nodes(), edge_list.num_arcs());
    eprintln!("");
}

/// computes distances with dijkstra
fn exercise1(args : &Args) {
    let (source, target) = report_time("Reading source target", || { read_source_target(&args.source_target)}); 
    let edge_list: EdgeList = report_time("Reading Graph", || {read_binary_graph(&args.graph, &args.weight).into()}); 
    report_graph(&edge_list);
    let array: AdjacencyArray= edge_list.into();
    let dij: DijkstraRunner<DirectedWeightedEdge, AdjacencyArray> = DijkstraRunner::new(&array);
    compute_distances(dij, &source, &target, &args.out_folder);
}

/// computes distances with ch and given augmented graph with ordering
fn exercise2(args : &Args) {
    let (source, target) = report_time("Reading source target", || { read_source_target(&args.source_target)}); 
    let edge_list: EdgeList = report_time("Reading Graph", || {read_binary_graph(&args.graph, &args.weight).into()}); 
    report_graph(&edge_list);
    let path_ordering: &String = args.ordering.as_ref().unwrap();
    let ordering: Vec<NodeId> = report_time("Reading Ordering", || {Vec::<NodeId>::load_from(path_ordering).unwrap()}); 
    let ch: CHGraphRunner = report_time("ch from augmented", || {CHGraphRunner::from_augmented_graph(edge_list, ordering)});
    compute_distances(ch, &source, &target, &args.out_folder);
}

/// computes distances with ch and given ordering
fn exercise3(args : &Args) {
    let (source, target) = report_time("Reading source target", || { read_source_target(&args.source_target)}); 
    let edge_list: EdgeList = report_time("Reading Graph", || {read_binary_graph(&args.graph, &args.weight).into()}); 
    report_graph(&edge_list);
    let path_ordering: &String = args.ordering.as_ref().unwrap();
    let config: ContractionConfig = ContractionConfig::new(args.witness_pre, args.witness_full);
    let ordering: Vec<NodeId> = report_time("Reading Ordering", || {Vec::<NodeId>::load_from(path_ordering).unwrap()}); 
    let ch: CHGraphRunner = report_time("ch from ordering", || {CHGraphRunner::from_ordering(edge_list, ordering, config)});
    compute_distances(ch, &source, &target, &args.out_folder);
}

/// computes distances with ch with bottom up construction
fn exercise4(args : &Args) {
    let (source, target) = report_time("Reading source target", || { read_source_target(&args.source_target)}); 
    let edge_list: EdgeList = report_time("Reading Graph", || {read_binary_graph(&args.graph, &args.weight).into()}); 
    report_graph(&edge_list);
    let config1: ContractionConfig = ContractionConfig::new(args.witness_pre, args.witness_full);
    let config2: BottomUpConfig = BottomUpConfig::new(args.lazy, args.update_interval, args.fraction_pops);
    let ch: CHGraphRunner = report_time("ch bottom up", || {CHGraphRunner::bottom_up(edge_list, config1, config2)});
    compute_distances(ch, &source, &target, &args.out_folder);
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    /// type of configuration for exercise with number [1 - 4]
    /// 1 -> Dijkstra, 
    /// 2 -> CH with given ordering and augmented graph, 
    /// 3 -> CH with given ordering, 
    /// 4 -> CH with bottom up construction, 
    exercise_nr: usize,
    
    #[clap(short, long)]
    /// folder with "first_out" and "head" file
    graph: String,
    
    #[clap(short, long)]
    /// file with edge weights corresponding to graph
    weight: String,
    
    #[clap(long)]
    /// file "ordering" with ordering that should be used in exercise 2 and 3 
    ordering: Option<String>,
    
    #[clap(short, long)]
    /// folder with a "source" and "target" file containing the source and target nodes
    source_target: String,

    #[clap(short, long)]
    /// file in which to write the distances between the source/target pairs
    out_folder: String,
    
    #[clap(long, default_value_t=1000)]
    /// maximal number of nodes visited in witness search for nodes in pq during bottom up construction
    witness_pre: usize,
    
    #[clap(long, default_value_t=10_000)]
    /// maximal number of nodes visited in witness search for nodes during contraction
    witness_full: usize,

    #[clap(long)]
    /// use lazy variant of bottom up construction
    lazy: bool,

    #[clap(long, default_value_t=1000)]
    /// length of intervals in lazy variant to check for updating all nodes
    update_interval: usize,
   
    #[clap(long, default_value_t=1.0f64)]
    /// trigger update in lazy variant if "fraction_pops" * "update_interval" <= successful pops
    fraction_pops: f64,
}

fn main(){
    let args = Args::parse();
    eprintln!("{:?}", args);

    match args.exercise_nr {
        1 => exercise1(&args),
        2 => exercise2(&args),
        3 => exercise3(&args),
        4 => exercise4(&args),
          _  => println!("Not a valid exercise"),
    }
}