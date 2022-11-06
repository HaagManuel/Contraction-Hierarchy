
use super::definitions::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DirectedWeightedEdge {
    pub to: NodeId,
    pub weight: Weight,
}
impl DirectedWeightedEdge {
    pub fn new() -> Self {
        DirectedWeightedEdge { to: 0, weight: 0 }
    }
    pub fn from_values(to: NodeId, weight: Weight) -> Self {
        DirectedWeightedEdge { to: to, weight: weight }
    }
}
impl Arc for DirectedWeightedEdge {
    fn head(&self) -> NodeId { self.to}
}
impl Weighted for DirectedWeightedEdge {
    fn weight(&self) -> Weight {self.weight } 
}


/* Nodes and Edges */