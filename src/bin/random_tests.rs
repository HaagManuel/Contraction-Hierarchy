use stud_rust_base::ch::ch_graph::CHGraph;
use stud_rust_base::ch::contraction::{BottomUpConfig, ContractionConfig};
use stud_rust_base::dijkstra::*;
use stud_rust_base::graph::{definitions::*, edge_list::{*}, nodes_edges::*, adjacency_array::AdjacencyArray};
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
        let edge_list: EdgeList = graph_gen::random_graph(num_nodes, num_edges);
        
        let mut data1: DijkstraData = DijkstraData::new(num_nodes);
        let mut data2: DijkstraData = DijkstraData::new(num_nodes);
        let mut data3: DijkstraData = DijkstraData::new(num_nodes);
        
        let array: AdjacencyArray = edge_list.clone().into();
        let (ch, _) = CHGraph::bottom_up_construction(edge_list, &mut data2, &mut data3, ContractionConfig::default(),             BottomUpConfig::default());

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

fn main() {
    test_random_bidir_all(200, 400, 3);
    test_random_ch_all(200, 500 , 1);
}
