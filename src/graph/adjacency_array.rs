
use super::definitions::*;
use super::edge_list::*;
use super::adjacency_list::*;
use super::nodes_edges::*;

pub struct AdjacencyArray {
    first_edge: Vec<NodeId>, // n + entires, last entry is dummy
    edge_arr: Vec<DirectedWeightedEdge>,
}

impl Graph for AdjacencyArray {
    fn num_nodes(&self) -> usize { self.first_edge.len() - 1}
    fn num_arcs(&self) -> usize { self.edge_arr.len()}
    fn degree(&self, node: NodeId) -> usize { 
        let n = node as usize;
        (self.first_edge[n + 1] - self.first_edge[n]) as usize
    }
}

impl From<AdjacencyList> for AdjacencyArray {
    fn from(adj_list: AdjacencyList) -> Self {
        //adjacency list must be sorted
        let n = adj_list.num_nodes();
        let mut _first_edge: Vec<NodeId> = vec![0; n + 1];
        let mut pref_sum: usize = 0;
        for i in 1..=n {
            pref_sum += adj_list.graph[i - 1].len();
            _first_edge[i] = pref_sum as NodeId;
        }
        let _edge_arr: Vec<DirectedWeightedEdge>  = adj_list.graph.concat();
        AdjacencyArray { first_edge: _first_edge, edge_arr: _edge_arr }
    }
}

impl From<EdgeList> for AdjacencyArray {
    fn from(edge_list: EdgeList) -> Self {
        let n = edge_list.num_nodes();
        let m = edge_list.num_arcs();
        let mut edges = edge_list.edges;
        edges.sort();
        let mut _first_edge: Vec<NodeId> = vec![0; n + 1];
        
        //compute degrees
        for (node, _) in &edges {
            _first_edge[*node as usize + 1] += 1;    
        }
        //prefix sum -> indices
        for i in 1..=n {
            _first_edge[i] += _first_edge[i - 1];
        }
        let mut _edge_arr: Vec<DirectedWeightedEdge> = Vec::new();
        _edge_arr.reserve_exact(m);
        for (_, e) in edges {
            _edge_arr.push(e);
        }
        
        AdjacencyArray { first_edge: _first_edge, edge_arr: _edge_arr }
    }
}

impl IncidentEdges<DirectedWeightedEdge> for AdjacencyArray {
    fn incident_edges(&self, node: NodeId) -> std::slice::Iter<DirectedWeightedEdge> {
        let a = self.first_edge[node as usize] as usize;
        let b = self.first_edge[(node + 1) as usize] as usize;
        self.edge_arr[a..b].iter()
    }
    fn mut_incident_edges(&mut self, node: NodeId) -> std::slice::IterMut<DirectedWeightedEdge> {
        let a = self.first_edge[node as usize] as usize;
        let b = self.first_edge[(node + 1) as usize] as usize;
        self.edge_arr[a..b].iter_mut()
    }
}
