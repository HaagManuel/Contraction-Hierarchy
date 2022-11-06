
use crate::dijkstra::*;

use crate::graph::adjacency_list::*;
use crate::graph::definitions::*;
use crate::graph::nodes_edges::*;
use crate::datastructure::index_heap::*;

// const MAX_WITNESS_SEARCH_STEPS: usize = 50_000;
const MAX_WITNESS_SEARCH_STEPS_HEURISTIC: usize = 100; //smaller witness search for heurstic for faster preprocessing
const MAX_WITNESS_SEARCH_STEPS: usize = 10_000;

pub struct Contraction<'a> {
    fwd_graph: &'a mut AdjacencyList,
    bwd_graph: &'a mut AdjacencyList,
    dij_data1: &'a mut DijkstraData,
    dij_data2: &'a mut DijkstraData,
    num_shortcuts: usize
}

struct Shortcut {
    from: NodeId,
    to: NodeId,
    weight: Weight,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct State { //for PQ
    pub heuristic: Weight,
    pub node: NodeId,
}
impl Indexing for State {
    fn as_index(&self) -> usize {
        self.node as usize
    }
}

pub struct LazyContractionConfig {
    update_interval: usize,
    fraction_pops: f64,
}

impl Default for LazyContractionConfig {
    fn default() -> Self {
        Self { update_interval: 1000, fraction_pops: 1.0f64 }
    }
}

impl<'a> Contraction<'a> {
    pub fn new(fwd_gr: &'a mut AdjacencyList, bwd_gr: &'a mut AdjacencyList, _dij_data1: &'a mut DijkstraData, _dij_data2: &'a mut DijkstraData) -> Self {
        Contraction { 
            fwd_graph: fwd_gr,
            bwd_graph: bwd_gr, 
            dij_data1: _dij_data1,
            dij_data2: _dij_data2,
            num_shortcuts: 0,
        }
    }

    //maybe reorder graph before
    pub fn contract_ordering(&mut self, ordering: Vec<NodeId>) {
        for j in ordering {
            assert!(j < self.fwd_graph.num_nodes() as u32, "{} >= {} !", j, self.fwd_graph.num_nodes());
            self.contract_node(j);
        }
    }

    //maybe only to higher order nodes? -> test with id ordering
    fn remove_incoming_edges(&mut self, node: NodeId) {
        let node_id: usize = node as usize;
        //remove v --> node in fwd
        for bwd in 0..self.bwd_graph.degree(node) { 
            let v: NodeId = self.bwd_graph.graph[node_id][bwd].head();
            self.fwd_graph.swap_remove_edge(v, node);
        }
        //remove v --> node in bwd
        for fwd in 0..self.fwd_graph.degree(node) {
            let v: NodeId = self.fwd_graph.graph[node_id][fwd].head();
            self.bwd_graph.swap_remove_edge(v, node);
        }
    }

    fn insert_shortcut(&mut self, shortcut: Shortcut) {
        self.fwd_graph.insert_or_decrease(shortcut.from, shortcut.to, shortcut.weight);
        self.bwd_graph.insert_or_decrease(shortcut.to, shortcut.from, shortcut.weight);
    }

    fn process_witness_search(&self, shortcut: &Shortcut, result: Option<Distance>) -> bool {
        match result {
            Some(dist) => {
                if shortcut.weight < dist { //shortcut preserves shortest paths, evtl. <= ?
                    return true;
                } else {return false;} //found existing shortest path that makes shortcut unnecessary 
            }
            None => { //did not find shortest path in cap
                return true; 
            }
        }
    }

    fn requires_shortcut(&mut self, shortcut: &Shortcut) -> bool {
        let mut fwd_dij: Dijkstra<DirectedWeightedEdge, AdjacencyList> = Dijkstra::new(self.fwd_graph, self.dij_data1);
        let mut bwd_dij: Dijkstra<DirectedWeightedEdge, AdjacencyList> = Dijkstra::new(self.bwd_graph, self.dij_data2);
        let config: DijkstraConfig = DijkstraConfig{
            start: shortcut.from,
            target: shortcut.to, 
            cap: Some(shortcut.weight),
            max_steps: Some(MAX_WITNESS_SEARCH_STEPS),
            forbidden_node: None,
        };
        let res = Dijkstra::generic_bidrektional_dijkstra::<SmallerQueue, WitnessSearchStop>(config, &mut fwd_dij, &mut bwd_dij); 
        return self.process_witness_search(shortcut, Some(res));
    }

    //incoming edges to node were node removed, deactivate node only for the search
    fn would_require_shortcut(&mut self, shortcut: &Shortcut, node: NodeId) -> bool {
        let mut fwd_dij: Dijkstra<DirectedWeightedEdge, AdjacencyList> = Dijkstra::new(self.fwd_graph, self.dij_data1);
        let mut bwd_dij: Dijkstra<DirectedWeightedEdge, AdjacencyList> = Dijkstra::new(self.bwd_graph, self.dij_data2);
        let config: DijkstraConfig = DijkstraConfig{
            start: shortcut.from,
            target: shortcut.to, 
            cap: Some(shortcut.weight),
            max_steps: Some(MAX_WITNESS_SEARCH_STEPS_HEURISTIC),
            forbidden_node: Some(node),
        };
        let res = Dijkstra::generic_bidrektional_dijkstra::<SmallerQueue, WitnessSearchStop>(config, &mut fwd_dij, &mut bwd_dij); 
        return self.process_witness_search(shortcut, Some(res));
    }

    pub fn contract_node(&mut self, node: NodeId) {
        self.remove_incoming_edges(node);
        let node_id = node as usize;
        //for all pairs of neighbors of node, check if shortcut is needed
        for bwd in 0..self.bwd_graph.degree(node) { 
            for fwd in 0..self.fwd_graph.degree(node) {
                let e_in: DirectedWeightedEdge = self.bwd_graph.graph[node_id][bwd];
                let e_out: DirectedWeightedEdge = self.fwd_graph.graph[node_id][fwd];
                let shortcut_weight = e_in.weight() + e_out.weight();
                let shortcut: Shortcut = Shortcut { from: e_in.head(), to: e_out.head(), weight: shortcut_weight };
                if self.requires_shortcut(&shortcut) {
                    self.insert_shortcut(shortcut);
                    self.num_shortcuts += 1;
                }
             }
        }
    }

    pub fn num_shortcuts_of_contraction(&mut self, node: NodeId) -> usize {
        let mut shortcuts: usize = 0; 
        let node_id = node as usize;
        for bwd in 0..self.bwd_graph.degree(node) { 
            for fwd in 0..self.fwd_graph.degree(node) {
                let e_in: DirectedWeightedEdge = self.bwd_graph.graph[node_id][bwd];
                let e_out: DirectedWeightedEdge = self.fwd_graph.graph[node_id][fwd];
                let shortcut_weight = e_in.weight() + e_out.weight();
                let shortcut: Shortcut = Shortcut { from: e_in.head(), to: e_out.head(), weight: shortcut_weight };
                if self.would_require_shortcut(&shortcut, node) {
                    shortcuts += 1;
                }
             }
        }
        return shortcuts;
    }

    pub fn net_gain_edges(&mut self, node: NodeId, level: &Vec<NodeId>) -> Weight {
        let new_edges = 2 * self.num_shortcuts_of_contraction(node) as Weight; //forward and backward edge
        let del_edges = (self.fwd_graph.degree(node) + self.bwd_graph.degree(node)) as Weight;
        return new_edges - del_edges + level[node as usize] as Weight; //net gain of edges + level
    }

    fn update_level(&self, v: NodeId, level: &mut Vec<NodeId>) {
        for fwd in 0..self.fwd_graph.degree(v) {
            level[fwd] = std::cmp::max(level[fwd], 1 + level[v as usize]);
        }
        for bwd in 0..self.bwd_graph.degree(v) {
            level[bwd] = std::cmp::max(level[bwd], 1 + level[v as usize]);
        }
    }

    pub fn bottom_up(&mut self, lazy: bool) -> Vec<NodeId> {
        if lazy {
            return self.lazy_bottom_up(LazyContractionConfig::default());
        } else {
            return self.simple_bottom_up();
        }
    }

    fn lazy_bottom_up(&mut self, config: LazyContractionConfig) -> Vec<NodeId> {
        let n: usize = self.fwd_graph.num_nodes();
        let mut ordering: Vec<NodeId> = Vec::new(); //unimportant nodes first
        let mut level: Vec<NodeId> = vec![0; n];
        ordering.reserve_exact(n);
        let mut heap = IndexdMinHeap::new(n);

        //init pq
        for i in 0..n {
            let w = i as NodeId;
            heap.push(State{heuristic: self.net_gain_edges(w, &level), node: i as NodeId});
        }

        //pop and assign node with min heuristic ~ low number of shortcuts
        let mut count_pops: usize = 0;
        let mut count_round: usize = 0;
        // let interval: usize = config.update_interval;
        // let frac: f64 = config.fraction_pops;
        let interval: usize = 10000;
        let frac: f64 = 1.0;
        // let frac: f64 = 0.75;
        while !heap.is_empty() {
            if ordering.len() % 10000 == 0 { println!("{}/{n}, pq {}, shortcuts {}", ordering.len(), heap.len(), self.num_shortcuts);}
            if count_round % interval == interval - 1 { 
                let lower: usize = ((interval as f64) * frac).floor() as usize;
                if count_pops >= lower { //update all nodes
                    println!("complete update {}", ordering.len());
                    for w in self.fwd_graph.nodes() {
                        if heap.contains_index(w as usize) {
                            heap.update_key(State { heuristic: self.net_gain_edges(w, &level), node: w });
                        }
                    }
                }
                count_pops = 0;
            }
            count_round += 1;

            //lazy update
            let v: NodeId = heap.peek().unwrap().node;
            let before: Weight = heap.peek().unwrap().heuristic;
            let after = self.net_gain_edges(v, &level);
            if before != after {
                heap.update_key(State{heuristic: after, node: v});
                continue;
            }
            heap.pop();
            ordering.push(v);
            self.contract_node(v);
            count_pops += 1;
            self.update_level(v, &mut level);
        }
        return ordering;
    }

    //computes ordering and applies contraction to graphs
    fn simple_bottom_up(&mut self) -> Vec<NodeId> {
        let n: usize = self.fwd_graph.num_nodes();
        let mut ordering: Vec<NodeId> = Vec::new(); //unimportant nodes first
        let mut level: Vec<NodeId> = vec![0; n];
        ordering.reserve_exact(n);

        #[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
        struct State {
            pub heuristic: Weight,
            pub node: NodeId,
        }
        impl Indexing for State {
            fn as_index(&self) -> usize {
                self.node as usize
            }
        }
        let mut heap = IndexdMinHeap::new(n);

        //init pq
        for i in 0..n {
            // if i % 1000 == 0 { println!("{i}/{n}");}
            let w = i as NodeId;
            let heuristic: Weight = self.net_gain_edges(w, &level);
            // let heuristic: f64 = self.relative_gain(w, &level);
            heap.push(State{heuristic: heuristic, node: i as NodeId});
        }
        //pop and assign node with min heuristic ~ low number of shortcuts
        let mut count_pops: usize = 0;
        let mut count_round: usize = 0;
        let interval: usize = 1000;
        while !heap.is_empty() {
            if ordering.len() % 10000 == 0 { println!("{}/{n}, pq {}, shortcuts {}", ordering.len(), heap.len(), self.num_shortcuts);}
            let v: NodeId = heap.pop().unwrap().node;
            let v_id: usize = v as usize;
            ordering.push(v);
            self.contract_node(v);
            self.update_level(v, &mut level);

            //update neighborhood
            for fwd in 0..self.fwd_graph.degree(v) {
                let w: NodeId = self.fwd_graph.graph[v_id][fwd].head();
                if heap.contains_index(w as usize) {
                    heap.update_key(State { heuristic: self.net_gain_edges(w, &level), node: w });
                }
            }
            for bwd in 0..self.bwd_graph.degree(v) {
                let w: NodeId = self.bwd_graph.graph[v_id][bwd].head();
                if heap.contains_index(w as usize) {
                    heap.update_key(State { heuristic: self.net_gain_edges(w, &level), node: w });
                }
            }
        }
        return ordering;
    }
}


#[cfg(test)]
mod tests {
    use crate::graph::edge_list::EdgeList;

    use super::*;

    #[test]
    fn test_graph_from_slide() {
        /*
            1
        4   0   2
            3
        */
        let edges: Vec<(NodeId, DirectedWeightedEdge)> = vec![
            (0, DirectedWeightedEdge::from_values(1, 4)),
            (0, DirectedWeightedEdge::from_values(3, 2)),
            (1, DirectedWeightedEdge::from_values(0, 3)),
            (2, DirectedWeightedEdge::from_values(0, 4)),
            (2, DirectedWeightedEdge::from_values(1, 7)),
            (3, DirectedWeightedEdge::from_values(0, 2)),
            (3, DirectedWeightedEdge::from_values(2, 3)),
            (4, DirectedWeightedEdge::from_values(0, 1)),
            (4, DirectedWeightedEdge::from_values(1, 6)),
        ];
        let edges_ch: Vec<(NodeId, DirectedWeightedEdge)> = vec![
            (0, DirectedWeightedEdge::from_values(1, 4)),
            (0, DirectedWeightedEdge::from_values(3, 2)),
            (1, DirectedWeightedEdge::from_values(3, 5)),
            (2, DirectedWeightedEdge::from_values(1, 7)),
            (2, DirectedWeightedEdge::from_values(3, 6)),
            (3, DirectedWeightedEdge::from_values(1, 6)),
            (3, DirectedWeightedEdge::from_values(2, 3)),
            (4, DirectedWeightedEdge::from_values(1, 5)),
            (4, DirectedWeightedEdge::from_values(3, 3)),
        ];
        let edge_list: EdgeList = edges.into();
        let rev_edge_list: EdgeList = edge_list.reverse_edge_list();
        let c_edge_list: EdgeList = edges_ch.into();

        let mut fwd: AdjacencyList = edge_list.clone().into();
        let mut bwd: AdjacencyList = rev_edge_list.clone().into();
        let mut dd: DijkstraData = DijkstraData::new(edge_list.num_nodes());
        let mut dd2: DijkstraData = DijkstraData::new(edge_list.num_nodes());

        let target_fwd: AdjacencyList = c_edge_list.clone().into();

        let mut contr: Contraction = Contraction::new(&mut fwd, &mut bwd, &mut dd, &mut dd2);
        contr.contract_node(0);
        fwd.sort_lists();
        bwd.sort_lists();

        assert_eq!(fwd, target_fwd);


    }


}