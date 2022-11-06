use std::env;
use std::path::Path;

use stud_rust_base::ch::ch_graph::{CHGraph, CHGraphRunner};
use stud_rust_base::dijkstra::*;
use stud_rust_base::graph::{definitions::*, edge_list::{*}, nodes_edges::*, adjacency_array::AdjacencyArray};

use stud_rust_base::io::*;
use stud_rust_base::time::report_time;

#[derive(Clone, Copy)]
enum Metric {
    TravelTime,
    GeoDistance
}

impl Metric {
    fn get_out_folder(&self)  -> &str{
        match *self {
            Self::TravelTime => "travel_time_length",
            Self::GeoDistance => "geo_distance_length",
        }
    }

    fn get_name(&self)  -> &str{
        match *self {
            Self::TravelTime => "travel_time",
            Self::GeoDistance => "geo_distance",
        }
    }

    fn read_graph(&self, path: &Path)  -> EdgeList{
        match *self {
            Self::TravelTime => read_binary_graph_travel(path).into(),
            Self::GeoDistance => read_binary_graph_geo(path).into(),
        }
    }
}

fn read_source_target(path: &Path) -> (Vec<NodeId>, Vec<NodeId>) {
    let dir: &Path = &Path::new(path);
    let source: Vec<NodeId> = Vec::<NodeId>::load_from(dir.join("test").join("source")).unwrap();
    let target: Vec<NodeId> = Vec::<NodeId>::load_from(dir.join("test").join("target")).unwrap();
    return (source, target);
}

fn write_result(path_out: &Path, result: Vec<Distance>, metric: Metric) {
    result.write_to(&path_out.join(metric.get_out_folder())).unwrap();
}

fn compute_distances<T: OneToOne>(mut algo: T, metric: Metric, source: &Vec<NodeId>, target: &Vec<NodeId>, path_out: &Path) {
    let mut result: Vec<Distance> = vec![3; source.len()];
    report_time(metric.get_name(), || {
        for (i, (s,t)) in std::iter::zip(source.iter(), target.iter()).enumerate() {
            if i % 10000 == 0 { eprintln!("{}/{}", i, source.len()); }
            result[i] = algo.one_to_one(*s, *t);
        }
    });
    eprintln!("");
    
    report_time("Write Results", || {
        eprintln!("{:?}", path_out);
        write_result(path_out, result, metric);
    });
    eprintln!("");
}

fn exercise1(path_in: &Path, path_out: &Path) {
    let (source, target) = report_time("Reading source target", || {read_source_target(path_in)}); 
    eprintln!(" ");
    
    let metric = Metric::TravelTime;
    let travel = report_time("Reading Graph", || { metric.read_graph(path_in)}); 
    eprintln!(" ");

    let gr_travel: AdjacencyArray = travel.into();
    let dij: DijkstraRunner<DirectedWeightedEdge, AdjacencyArray> = DijkstraRunner::new(&gr_travel);
    compute_distances(dij, metric, &source, &target, path_out);

    let metric = Metric::GeoDistance;
    let travel = report_time("Reading Graph", || {metric.read_graph(path_in)}); 
    eprintln!(" ");

    let gr_geo: AdjacencyArray = travel.into();
    let dij: DijkstraRunner<DirectedWeightedEdge, AdjacencyArray> = DijkstraRunner::new(&gr_geo);
    compute_distances(dij, metric, &source, &target, path_out);

}

fn exercise2(path_in: &Path, path_out: &Path) {
    // let (travel, geo, source, target) = read_input(path_in);
    // let n: usize = travel.num_nodes();
    // let mut dij_data1: DijkstraData = DijkstraData::new(n);
    // let mut dij_data2: DijkstraData = DijkstraData::new(n);
    // let mut res_travel: Vec<Distance> = vec![0; n];
    // let mut res_geo: Vec<Distance> = vec![0; n];
}

fn exercise3(path_in: &Path, path_out: &Path) {
    // let (travel, geo, source, target) = read_input(path_in);
    // let n: usize = travel.num_nodes();
    // let mut dij_data1: DijkstraData = DijkstraData::new(n);
    // let mut dij_data2: DijkstraData = DijkstraData::new(n);
    // let mut res_travel: Vec<Distance> = vec![0; n];
    // let mut res_geo: Vec<Distance> = vec![0; n];
}

fn exercise4(path_in: &Path, path_out: &Path) {
    let (source, target) = report_time("Reading source target", || { read_source_target(path_in)}); 
    eprintln!(" ");
    
    let metric = Metric::TravelTime;
    let travel = report_time("Reading Graph", || {metric.read_graph(path_in)}); 
    eprintln!(" ");
    let ch: CHGraphRunner = report_time("ch bottom up", || {CHGraphRunner::bottom_up(travel)});
    eprintln!(" ");
    compute_distances(ch, metric, &source, &target, path_out);
    eprintln!(" ");
    
    let metric = Metric::GeoDistance;
    let geo = report_time("Reading Graph", || {metric.read_graph(path_in)}); 
    eprintln!(" ");
    let ch: CHGraphRunner = report_time("ch bottom up", || {CHGraphRunner::bottom_up(geo)});
    eprintln!(" ");
    compute_distances(ch, metric, &source, &target, path_out);
    eprintln!(" ");
}


fn main(){
    println!("arguments: [a1 | a2 | a3 | a4] [path_in] [path_out]");

    let _arg0: &String = &env::args().nth(0).unwrap();
    let exercise: &String = &env::args().nth(1).expect("No exercise type given");
    let arg2: &String = &env::args().nth(2).expect("No input path given");
    let arg3: &String = &env::args().nth(3).expect("No output path given");

    let path_in: &Path = Path::new(arg2);
    let path_out: &Path = Path::new(arg3);

    // println!("{} {} {} {}", _arg0, exercise, arg2, arg3); 
    // return;

    match exercise.as_ref() {
        "a1" => exercise1(path_in, path_out),
        "a2" => exercise2(path_in, path_out),
        "a3" => exercise3(path_in, path_out),
        "a4" => exercise4(path_in, path_out),
          _  => println!("Not a valid exercise"),
    }
}