use crate::dijkstra::*;
use crate::graph::adjacency_array::{self, AdjacencyArray};
use crate::graph::adjacency_list::{self, AdjacencyList};
use crate::graph::edge_list::*;
use crate::graph::nodes_edges::*;
use crate::graph::*;
use crate::graph::definitions::*;

use super::contraction::{Contraction};

pub struct CHGraph {
    fwd_graph: AdjacencyArray,
    bwd_graph: AdjacencyArray,
}

impl CHGraph {
    pub fn from_ordering(edge_list: EdgeList, ordering: Vec<NodeId>, dij_data1: &mut DijkstraData, dij_data2: &mut DijkstraData) -> Self {
        let rev_edge_list: EdgeList = edge_list.reverse_edge_list();

        let mut fwd_list: AdjacencyList = edge_list.into();
        let mut bwd_list: AdjacencyList = rev_edge_list.into();

        let mut builder: Contraction = Contraction::new(&mut fwd_list, &mut bwd_list, dij_data1, dij_data2);
        builder.contract_ordering(ordering);
        
        fwd_list.sort_lists(); // sorting is destroyed during contraction
        bwd_list.sort_lists();

        // println!("fwd ch");
        // fwd_list.print();
        // println!(" ");

        // println!("bwd ch");
        // bwd_list.print();
        // println!(" ");

        CHGraph{ fwd_graph: fwd_list.into(), bwd_graph: bwd_list.into() }
    }

    pub fn bottom_up_construction(edge_list: EdgeList, dij_data1: &mut DijkstraData, dij_data2: &mut DijkstraData) -> (Self, Vec<NodeId>) {
        let rev_edge_list: EdgeList = edge_list.reverse_edge_list();

        let mut fwd_list: AdjacencyList = edge_list.into();
        let mut bwd_list: AdjacencyList = rev_edge_list.into();

        let mut builder: Contraction = Contraction::new(&mut fwd_list, &mut bwd_list, dij_data1, dij_data2);
        // let ordering = builder.simple_bottom_up();
        let ordering = builder.bottom_up(true);
        
        fwd_list.sort_lists(); // sorting is destroyed during contraction
        bwd_list.sort_lists();
        
        // println!("fwd ch");
        // fwd_list.print();
        // println!(" ");

        // println!("bwd ch");
        // bwd_list.print();
        // println!(" ");

        (CHGraph{ fwd_graph: fwd_list.into(), bwd_graph: bwd_list.into()}, ordering)
    }


    pub fn one_to_one(&self, start: NodeId, target: NodeId, dij_data1: &mut DijkstraData, dij_data2: &mut DijkstraData) -> Distance {
        let mut fwd_dij: Dijkstra<DirectedWeightedEdge, AdjacencyArray> = Dijkstra::new(&self.fwd_graph, dij_data1);
        let mut bwd_dij: Dijkstra<DirectedWeightedEdge, AdjacencyArray> = Dijkstra::new(&self.bwd_graph, dij_data2);
        // return Dijkstra::bidrektional_dijkstra::<Alternating, CHSearchStop>(start, target, &mut fwd_dij, &mut bwd_dij);
        return Dijkstra::bidrektional_dijkstra::<SmallerQueueKey, CHSearchStop>(start, target, &mut fwd_dij, &mut bwd_dij);
    }

    
    pub fn print_degrees(&self) {
        let mut v: Vec<usize> = Vec::new();
        for i in self.fwd_graph.nodes() {
            v.push(self.fwd_graph.degree(i));
        }

        let mut w: Vec<usize> = vec![0; v.iter().max().unwrap() + 1];
        for x in v {
            w[x] += 1;
        }
        for (i, x) in w.iter().enumerate() {
            if *x > 0 {
                println!("deg {}, count {}", i, x);
            }
        }
    }
}

pub struct CHGraphRunner {
    data1: DijkstraData,
    data2: DijkstraData,
    ch: CHGraph,
}

impl CHGraphRunner {
    pub fn from_ordering(edge_list: EdgeList, ordering: Vec<NodeId>) -> Self {
        let mut data1: DijkstraData = DijkstraData::new(edge_list.num_nodes());
        let mut data2: DijkstraData = DijkstraData::new(edge_list.num_nodes());
        let ch: CHGraph =  CHGraph::from_ordering(edge_list, ordering, &mut data1, &mut data2);
        CHGraphRunner { 
            data1: data1, 
            data2: data2, 
            ch: ch}
    }

    pub fn bottom_up(edge_list: EdgeList) -> Self {
        let mut data1: DijkstraData = DijkstraData::new(edge_list.num_nodes());
        let mut data2: DijkstraData = DijkstraData::new(edge_list.num_nodes());
        let (ch, _) =  CHGraph::bottom_up_construction(edge_list, &mut data1, &mut data2);
        CHGraphRunner { 
            data1: data1, 
            data2: data2, 
            ch: ch}
    }
}

impl OneToOne for CHGraphRunner {
    fn one_to_one(&mut self, start: NodeId, target: NodeId) -> Distance {
        return self.ch.one_to_one(start, target, &mut self.data1, &mut self.data2);
    }
}