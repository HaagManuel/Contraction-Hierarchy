
//i dont get iterators to work -> i use slices instead

/* Type Definitions */
pub type NodeId = u32;
pub type EdgeId = u32;
pub type Weight = i32;
pub type Distance = i32;
pub type Coordinate = i64;
/* Type Definitions */


/* Trait Definitions */
pub trait Arc {
    fn head(&self) -> NodeId;
}
pub trait Weighted {
    fn weight(&self) -> Weight;
}

pub trait Graph {
    fn num_nodes(&self) -> usize;
    fn num_arcs(&self) -> usize;
    fn nodes(&self) -> std::ops::Range<NodeId> {0..self.num_nodes() as NodeId}
    fn edges(&self) -> std::ops::Range<EdgeId> {0..self.num_arcs() as EdgeId}
    fn degree(&self, node: NodeId) -> usize;
}

// Get an iterator over the outgoing links of the given node.
pub trait IncidentEdges<Edge> {
    fn incident_edges(&self, node: NodeId) -> std::slice::Iter<Edge>;
    fn mut_incident_edges(&mut self, node: NodeId) -> std::slice::IterMut<Edge>;
}


// Get an iterator over all edges.
pub trait IterableEdges<Edge> {
    fn iter_edges(&self) -> std::slice::Iter<Edge>;
    fn mut_iter_edges(&mut self) -> std::slice::IterMut<Edge>;
}
/* Trait Definitions */