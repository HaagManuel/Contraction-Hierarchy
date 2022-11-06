use stud_rust_base::{graph::{definitions::*, edge_list::{*}, adjacency_array::AdjacencyArray}, dijkstra::*};
use stud_rust_base::{ch::ch_graph::CHGraphRunner};
use stud_rust_base::benchmark;
use std::{env, path::Path};
use stud_rust_base::{time::report_time};


fn benchmark_random(edge_list: EdgeList, num_start: usize) {
    let array: AdjacencyArray = edge_list.clone().into();
    println!("n {}, m {}", edge_list.num_nodes(), edge_list.num_arcs());

    println!("Dijkstra");
    let runner = DijkstraRunner::new(&array);
    let result = benchmark::one_to_one_random(runner, edge_list.clone(), num_start);
    benchmark::eval_result(&result);
    println!(" ");
    

    // println!("Bidirectional Alternating");
    // let runner = BidirectionalRunner::<Alternating, SumQueueKeyStop>::new(edge_list.clone());
    // let result = benchmark::one_to_one_random(runner, edge_list.clone(), num_start);
    // benchmark::eval_result(&result);
    // println!(" ");

    //fasted on NY graph
    println!("Bidirectional SmallerQueue");
    let runner = BidirectionalRunner::<SmallerQueue, SumQueueKeyStop>::new(edge_list.clone());
    let result = benchmark::one_to_one_random(runner, edge_list.clone(), num_start);
    benchmark::eval_result(&result);
    println!(" ");

    // println!("Bidirectional SmallerQueueKey");
    // let runner = BidirectionalRunner::<SmallerQueueKey, SumQueueKeyStop>::new(edge_list.clone());
    // let result = benchmark::one_to_one_random(runner, edge_list.clone(), num_start);
    // benchmark::eval_result(&result);
    // println!(" ");
    
    let runner: CHGraphRunner = report_time("CH bottom up construction", || {
        return CHGraphRunner::bottom_up(edge_list.clone());
    });
    let result = benchmark::one_to_one_random(runner, edge_list.clone(), num_start);
    benchmark::eval_result(&result);
    println!(" ");
}


fn main(){
    println!("arguments: [a | bin_travel | bin_geo] [directorypath] [num_startpoints]");

    macro_rules! print_param {
        ($a:expr, $b:expr) => {
            println!( "{0: <15} --> {1: <15}", $a, $b);
        };
    }

    print_param!("a", "Adjacencylist Format");
    print_param!("btravel", "Binary Format travel time");
    print_param!("bgeo", "Binary Format geo distances");
    print_param!("num_startpoints", "number of random start points");
    println!("");

    let variant = &env::args().nth(1).expect("No graphformat given");
    let path = &env::args().nth(2).expect("No directory given");
    let num_start: usize = env::args().nth(3).expect("No num_startpoints given").parse::<usize>().unwrap();

    let edge_list: EdgeList;

    if variant == "a" {
        edge_list = stud_rust_base::io::from_file_weighted(path).into();
    } else if variant == "btravel" {
        edge_list = stud_rust_base::io::read_binary_graph_travel(Path::new(path)).into();
    } else if variant == "bgeo" {
        edge_list = stud_rust_base::io::read_binary_graph_geo(Path::new(path)).into();
    } else {
        println!("{} not existing graphformat", variant);
        return;
    }
    benchmark_random(edge_list, num_start)
}


// fn main() {
//     let weighted = "../inputs/USA-road-d.NY.gr";
//     let edge_list: EdgeList = io::from_file_weighted(weighted).into();
// }
