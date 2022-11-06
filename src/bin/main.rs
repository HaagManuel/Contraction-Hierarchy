// https://networkrepository.com/road.php
// https://www.diag.uniroma1.it//~challenge9/download.shtml
// graphs, raw edges, all undirected and once in input

// road, no n m, weigthed
// USA NY - no n m, weigthed (n = 264346, m = 733846 )

// mod graph;
// mod dijkstra;
// mod datastructure;
// mod ch;
// mod io;
// mod benchmark;
// mod graph_gen;

use std::{fmt::Result, fs::File, io::Write, time::{Instant, Duration}};



use stud_rust_base::ch::ch_graph::CHGraph;
use stud_rust_base::dijkstra::*;
use stud_rust_base::graph::{definitions::*, edge_list::{*}, nodes_edges::*, adjacency_list::*, adjacency_array::AdjacencyArray};

use stud_rust_base::{dijkstra::{SmallerQueueKey, ZeroPotential}, ch::ch_graph::CHGraphRunner};

use stud_rust_base::benchmark;
use stud_rust_base::io;
use stud_rust_base::graph_gen;


//tests all pairs
fn test_random_bidir_all(num_nodes: usize, num_edges: usize, repetitions: usize) {
    assert!(num_edges * num_edges <= 200000); //only small graphs for all pairs
    for _ in 0..repetitions {
        let edge_list: EdgeList = graph_gen::random_graph(num_nodes, num_edges);
        let rev_edge_list: EdgeList = edge_list.reverse_edge_list();
        
        let graph: AdjacencyArray = edge_list.into();
        let rev_graph: AdjacencyArray = rev_edge_list.into();

        let mut data1: DijkstraData = DijkstraData::new(num_nodes);
        let mut data2: DijkstraData = DijkstraData::new(num_nodes);

        let mut fwd_dij: Dijkstra<DirectedWeightedEdge, AdjacencyArray> = Dijkstra::new(&graph, &mut data1);
        let mut bwd_dij: Dijkstra<DirectedWeightedEdge, AdjacencyArray> = Dijkstra::new(&rev_graph, &mut data2);

        let pot: ZeroPotential = ZeroPotential {};
        for s in 0..(num_nodes as NodeId - 1) {
            for t in 0..(num_nodes as NodeId - 1) {
                assert_eq!(
                    fwd_dij.one_to_one(s, t, &pot), 
                    Dijkstra::bidrektional_dijkstra::<SmallerQueueKey, SumQueueKeyStop>(s, t, &mut fwd_dij, &mut bwd_dij)
                );
            }
        }
    }
    println!("test bidirectional_dijkstra -> ok")
}

//tests all pairs
fn test_random_ch_all(num_nodes: usize, num_edges: usize, repetitions: usize) {
    assert!(num_nodes * num_nodes <= 1_000_000); //only small graphs for all pairs
    for _i in 0..repetitions {
        println!("i {_i} ");
        let edge_list: EdgeList = graph_gen::random_graph(num_nodes, num_edges);
        // let edge_list: EdgeList = graph_gen::directed_path(num_nodes);
        // let edge_list: EdgeList = graph_gen::directed_path(num_nodes).reverse_edge_list();
        
        let mut data1: DijkstraData = DijkstraData::new(num_nodes);
        let mut data2: DijkstraData = DijkstraData::new(num_nodes);
        let mut data3: DijkstraData = DijkstraData::new(num_nodes);
        
        let array: AdjacencyArray = edge_list.clone().into();
        let list: AdjacencyList = edge_list.clone().into();
        // let ordering: Vec<NodeId> = (0..(num_nodes as NodeId - 1)).collect();
        // let ch: CHGraph = CHGraph::from_ordering(edge_list, ordering, &mut data2, &mut data3);
        let (ch, _) = CHGraph::bottom_up_construction(edge_list, &mut data2, &mut data3);

        // println!("list");
        // list.print();

        let mut dij: Dijkstra<DirectedWeightedEdge, AdjacencyArray> = Dijkstra::new(&array, &mut data1);
        let pot: ZeroPotential = ZeroPotential {};

        for s in 0..(num_nodes as NodeId - 1) {
            for t in 0..(num_nodes as NodeId - 1) {
                assert_eq!(
                    dij.one_to_one(s, t, &pot), 
                    ch.one_to_one(s, t, &mut data2, &mut data3),
                    "{} {}", s, t
                );
            }
        }
    }
    println!("ch test -> ok");
}

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
    
    
    // println!("CH bottom up ordering");
    // let now: Instant = Instant::now();
    // let runner = CHGraphRunner::bottom_up(edge_list.clone());
    // let elapsed: Duration = now.elapsed();    
    // println!("CH bottom up construction time: {:?}", elapsed);
    // let result = benchmark::one_to_one_random(runner, edge_list.clone(), num_start);
    // benchmark::eval_result(&result);
    // println!(" ");
}

fn write_ordering(ordering: Vec<NodeId>, path: &str) {
    let data: Vec<String> = ordering.iter().map(|&x| x.to_string()).collect();
    let mut out = data.join(" ");
    out.push('\n');
    std::fs::write(path, out).expect("Failed to read to file {path}");
}

fn read_ordering(path: &str) -> Vec<NodeId> {
    let mut contents = std::fs::read_to_string(path).expect("Failed to read file {path}");
    contents.truncate(contents.len() - 1); //remove \n
    return contents.split(" ").map(|s| s.parse::<NodeId>().expect(s)).collect();
}

fn main() {
    println!("Hello, graph!");

    // let path = "../inputs/ch-USA-road-d.NY.txt";

    // test_random_bidir_all(200, 400, 25);
    // test_random_ch_all(200, 500 , 1);
    // benchmark::benchmark_array_vs_list(edge_list);
    // let v: Vec<NodeId> = vec![0,4];
    // write_ordering(v, "../inputs/ordering_test.txt");
    
    // let n: usize = 100000;
    // let n: usize = 1000;
    // let edge_list: EdgeList = graph_gen::random_graph(n, 8 * n);
    
    let weighted = "../inputs/USA-road-d.NY.gr";
    // let coords_path = "../inputs/USA-road-d.NY.co";
    
    // let edge_list: EdgeList = graph_gen::random_graph(n, 8 * n);
    let edge_list: EdgeList = io::from_file_weighted(weighted).into();
    // let _coord = io::read_coordinates(coords_path);
    // let ordering = read_ordering(path);
    
    // benchmark::ch_from_ordering(edge_list, ordering);
    // benchmark::ch_bottom_up(edge_list);
    stud_rust_base::benchmark::ch_bottom_up(edge_list);
    
    // let num_start: usize = 100;
    // benchmark_random(edge_list, num_start);
    
    
    
    //takes ~ 878.461489234s
    //shorter preliminary witness search + levels -> shortcuts 817434, 273.190515904s, directly from ordering: 207.023004466s


    // let now: Instant = Instant::now();
    // let mut d1 = DijkstraData::new(edge_list.num_nodes());
    // let mut d2 = DijkstraData::new(edge_list.num_nodes());
    // let (ch, ordering) = CHGraph::bottom_up_construction(edge_list, &mut d1, &mut d2);
    // write_ordering(ordering, "../inputs/ch-USA-road-d.NY.txt");
    // let elapsed: Duration = now.elapsed();    
    // println!("{:?}", elapsed);
    // ch.print_degrees();

    // benchmark::ch_bottom_up(edge_list);

}

/*
USA-road-d.NY.gr, 100 random starts

n 264346, m 1467692
Dijkstra
2^7: 0.0010 [ms] +/- 0.0005 [ms]
2^8: 0.0009 [ms] +/- 0.0005 [ms]
2^9: 0.0010 [ms] +/- 0.0012 [ms]
2^10: 0.0006 [ms] +/- 0.0004 [ms]
2^11: 0.0008 [ms] +/- 0.0005 [ms]
2^12: 0.0022 [ms] +/- 0.0014 [ms]
2^13: 0.0072 [ms] +/- 0.0052 [ms]
2^14: 0.0269 [ms] +/- 0.0210 [ms]
2^15: 0.1048 [ms] +/- 0.0708 [ms]
2^16: 0.4284 [ms] +/- 0.2756 [ms]
2^17: 1.6481 [ms] +/- 1.0227 [ms]
2^18: 5.7816 [ms] +/- 3.5942 [ms]
2^19: 18.1382 [ms] +/- 8.3808 [ms]
2^20: 31.8816 [ms] +/- 6.4010 [ms]
 
Bidirectional Alternating
2^5: 0.0015 [ms] +/- 0.0005 [ms]
2^6: 0.0010 [ms] +/- 0.0000 [ms]
2^7: 0.0014 [ms] +/- 0.0006 [ms]
2^8: 0.0015 [ms] +/- 0.0007 [ms]
2^9: 0.0011 [ms] +/- 0.0006 [ms]
2^10: 0.0006 [ms] +/- 0.0004 [ms]
2^11: 0.0010 [ms] +/- 0.0007 [ms]
2^12: 0.0020 [ms] +/- 0.0013 [ms]
2^13: 0.0062 [ms] +/- 0.0046 [ms]
2^14: 0.0192 [ms] +/- 0.0112 [ms]
2^15: 0.1014 [ms] +/- 0.2945 [ms]
2^16: 0.2437 [ms] +/- 0.1288 [ms]
2^17: 0.9400 [ms] +/- 0.5296 [ms]
2^18: 3.2664 [ms] +/- 1.6298 [ms]
2^19: 11.1752 [ms] +/- 4.4043 [ms]
2^20: 19.8661 [ms] +/- 2.9084 [ms]
 
Bidirectional SmallerQueue
2^5: 0.0009 [ms] +/- 0.0002 [ms]
2^6: 0.0012 [ms] +/- 0.0009 [ms]
2^7: 0.0008 [ms] +/- 0.0002 [ms]
2^8: 0.0012 [ms] +/- 0.0006 [ms]
2^9: 0.0013 [ms] +/- 0.0024 [ms]
2^10: 0.0007 [ms] +/- 0.0004 [ms]
2^11: 0.0009 [ms] +/- 0.0008 [ms]
2^12: 0.0016 [ms] +/- 0.0009 [ms]
2^13: 0.0053 [ms] +/- 0.0045 [ms]
2^14: 0.0161 [ms] +/- 0.0100 [ms]
2^15: 0.0550 [ms] +/- 0.0354 [ms]
2^16: 0.2044 [ms] +/- 0.1335 [ms]
2^17: 0.7649 [ms] +/- 0.4529 [ms]
2^18: 2.7288 [ms] +/- 1.5436 [ms]
2^19: 9.3421 [ms] +/- 3.4983 [ms]
2^20: 17.0328 [ms] +/- 3.3610 [ms]
 
Bidirectional SmallerQueueKey
2^5: 0.0008 [ms] +/- 0.0000 [ms]
2^6: 0.0016 [ms] +/- 0.0003 [ms]
2^7: 0.0017 [ms] +/- 0.0017 [ms]
2^8: 0.0009 [ms] +/- 0.0004 [ms]
2^9: 0.0012 [ms] +/- 0.0026 [ms]
2^10: 0.0006 [ms] +/- 0.0004 [ms]
2^11: 0.0009 [ms] +/- 0.0007 [ms]
2^12: 0.0020 [ms] +/- 0.0010 [ms]
2^13: 0.0057 [ms] +/- 0.0031 [ms]
2^14: 0.0175 [ms] +/- 0.0102 [ms]
2^15: 0.0656 [ms] +/- 0.0473 [ms]
2^16: 0.2444 [ms] +/- 0.1366 [ms]
2^17: 0.9267 [ms] +/- 0.4723 [ms]
2^18: 3.5936 [ms] +/- 1.5338 [ms]
2^19: 12.9844 [ms] +/- 3.5364 [ms]
2^20: 22.6020 [ms] +/- 4.1577 [ms]


*/

/*n 264346, m 1467692
Dijkstra
2^6: 0.0004 [ms] +/- 0.0000 [ms]
2^7: 0.0005 [ms] +/- 0.0001 [ms]
2^8: 0.0014 [ms] +/- 0.0022 [ms]
2^9: 0.0006 [ms] +/- 0.0003 [ms]
2^10: 0.0004 [ms] +/- 0.0002 [ms]
2^11: 0.0007 [ms] +/- 0.0004 [ms]
2^12: 0.0020 [ms] +/- 0.0026 [ms]
2^13: 0.0065 [ms] +/- 0.0050 [ms]
2^14: 0.0219 [ms] +/- 0.0146 [ms]
2^15: 0.0859 [ms] +/- 0.0535 [ms]
2^16: 0.3495 [ms] +/- 0.2334 [ms]
2^17: 1.3175 [ms] +/- 0.7867 [ms]
2^18: 4.8077 [ms] +/- 2.7763 [ms]
2^19: 15.1043 [ms] +/- 6.6422 [ms]
2^20: 26.2417 [ms] +/- 2.3843 [ms]
 
Bidirectional SmallerQueue
2^3: 0.0009 [ms] +/- 0.0000 [ms]
2^4: 0.0006 [ms] +/- 0.0004 [ms]
2^5: 0.0014 [ms] +/- 0.0004 [ms]
2^7: 0.0011 [ms] +/- 0.0005 [ms]
2^8: 0.0013 [ms] +/- 0.0006 [ms]
2^9: 0.0011 [ms] +/- 0.0006 [ms]
2^10: 0.0008 [ms] +/- 0.0005 [ms]
2^11: 0.0010 [ms] +/- 0.0006 [ms]
2^12: 0.0018 [ms] +/- 0.0009 [ms]
2^13: 0.0050 [ms] +/- 0.0035 [ms]
2^14: 0.0147 [ms] +/- 0.0101 [ms]
2^15: 0.0484 [ms] +/- 0.0315 [ms]
2^16: 0.1778 [ms] +/- 0.1226 [ms]
2^17: 0.7427 [ms] +/- 0.4617 [ms]
2^18: 2.8248 [ms] +/- 1.6226 [ms]
2^19: 8.9974 [ms] +/- 3.6417 [ms]
2^20: 17.4893 [ms] +/- 3.1586 [ms]
 
CH bottom up ordering
CH bottom up construction time: 281.967515644s
2^5: 0.0010 [ms] +/- 0.0000 [ms]
2^7: 0.0016 [ms] +/- 0.0005 [ms]
2^8: 0.0021 [ms] +/- 0.0028 [ms]
2^9: 0.0019 [ms] +/- 0.0042 [ms]
2^10: 0.0013 [ms] +/- 0.0030 [ms]
2^11: 0.0012 [ms] +/- 0.0007 [ms]
2^12: 0.0024 [ms] +/- 0.0014 [ms]
2^13: 0.0051 [ms] +/- 0.0032 [ms]
2^14: 0.0111 [ms] +/- 0.0078 [ms]
2^15: 0.0275 [ms] +/- 0.0211 [ms]
2^16: 0.0605 [ms] +/- 0.0489 [ms]
2^17: 0.1414 [ms] +/- 0.0973 [ms]
2^18: 0.2988 [ms] +/- 0.1661 [ms]
2^19: 0.4985 [ms] +/- 0.1541 [ms]
2^20: 0.8731 [ms] +/- 0.1653 [ms]
*/