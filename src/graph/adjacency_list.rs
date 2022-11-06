
use super::definitions::*;
use super::edge_list::*;
use super::nodes_edges::*;

#[derive(Debug, PartialEq, Eq)]
pub struct AdjacencyList {
    pub graph: Vec<Vec<DirectedWeightedEdge>>,
    arcs: usize,
}

impl Graph for AdjacencyList {
    fn num_nodes(&self) -> usize { self.graph.len()}
    fn num_arcs(&self) -> usize { self.arcs}
    fn degree(&self, node: NodeId) -> usize { self.graph[node as usize].len() }
}

impl From<EdgeList> for AdjacencyList {
    fn from(edge_list: EdgeList) -> Self {
        let mut gr: Vec<Vec<DirectedWeightedEdge>> = vec![Vec::new(); edge_list.num_nodes()];
        let n = edge_list.num_nodes();
        let m = edge_list.num_arcs();
        for i in 0..n {
            gr[i].reserve_exact(edge_list.degree(i as NodeId));
        }
        for (from, e) in edge_list.edges {
            gr[from as usize].push(e);
        }
        for i in 0..n {
            gr[i as usize].sort();
        }
        let mut adj_list: AdjacencyList = AdjacencyList{graph: gr, arcs: m};
        adj_list.sort_lists();
        adj_list
    }
}

impl IncidentEdges<DirectedWeightedEdge> for AdjacencyList {
    fn incident_edges(&self, node: NodeId) -> std::slice::Iter<DirectedWeightedEdge> {self.graph[node as usize].iter()}
    fn mut_incident_edges(&mut self, node: NodeId) -> std::slice::IterMut<DirectedWeightedEdge> {self.graph[node as usize].iter_mut()}
}

impl AdjacencyList {
    //panics if element not present
    pub fn swap_remove_edge(&mut self, from: NodeId, to: NodeId) {
        let index = self.graph[from as usize].iter().position(|&e| e.head() == to);
        self.graph[from as usize].swap_remove(index.unwrap());
    }
    
    pub fn remove_edge(&mut self, from: NodeId, to: NodeId) {
        let index = self.graph[from as usize].iter().position(|&e| e.head() == to);
        self.graph[from as usize].remove(index.unwrap());
    }
    //insert a new edge at the end if the edge is not present, otherwise updates weight if it is strictly less than present weight
    pub fn insert_or_decrease(&mut self, from: NodeId, to: NodeId, weight: Weight) {
        let v = from as usize;
        let index = self.graph[v].iter().position(|&e| e.head() == to);
        if let Some(i) = index {
            self.graph[v][i].weight = std::cmp::min(weight, self.graph[v][i].weight);
        }
        else {
            self.graph[v].push(DirectedWeightedEdge { to: to, weight: weight })
        }
    }

    pub fn sort_lists(&mut self) {
        for i in 0..self.num_nodes() {
            self.graph[i as usize].sort();
        }
    }

    pub fn print(&self) {
        for i in 0..self.num_nodes() {
            println!("{:?}", self.graph[i])
        }
    }
}
