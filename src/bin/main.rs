


use stud_rust_base::ch::ch_graph::CHGraph;
use stud_rust_base::dijkstra::*;
use stud_rust_base::graph::{definitions::*, edge_list::{*}, nodes_edges::*, adjacency_list::*, adjacency_array::AdjacencyArray};

use stud_rust_base::{dijkstra::{SmallerQueueKey, ZeroPotential}, ch::ch_graph::CHGraphRunner};

use stud_rust_base::benchmark;
use stud_rust_base::io;





fn main() {
    let weighted = "../inputs/USA-road-d.NY.gr";
    let edge_list: EdgeList = io::from_file_weighted(weighted).into();

    // let now: Instant = Instant::now();
    // let mut d1 = DijkstraData::new(edge_list.num_nodes());
    // let mut d2 = DijkstraData::new(edge_list.num_nodes());
    // let (ch, ordering) = CHGraph::bottom_up_construction(edge_list, &mut d1, &mut d2);
    // write_ordering(ordering, "../inputs/ch-USA-road-d.NY.txt");
    // let elapsed: Duration = now.elapsed();    
    // println!("{:?}", elapsed);
    // ch.print_degrees();


}
