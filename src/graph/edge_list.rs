
use super::{definitions::*, nodes_edges::DirectedWeightedEdge};
#[derive(Clone)]
pub struct EdgeList {
    num_nodes: usize,
    num_edges: usize,
    degree: Vec<NodeId>,
    pub edges: Vec<(NodeId, DirectedWeightedEdge)>, // node from which edge is outgoing
}


impl EdgeList {
    pub fn reverse_edge_list(&self) -> EdgeList {
        let mut rev_edges: Vec<(NodeId, DirectedWeightedEdge)> = vec![(0, DirectedWeightedEdge::new()); self.num_edges]; 
        let mut rev_degree: Vec<NodeId> = vec![0; self.num_nodes];
        for i in 0..self.num_edges {
            let (from, e) = self.edges[i];
            rev_edges[i] = (e.to, DirectedWeightedEdge::from_values(from, e.weight()));
            rev_degree[e.to as usize] += 1;
        }

        EdgeList { num_nodes: self.num_nodes, num_edges: self.num_edges, degree: rev_degree, edges: rev_edges }
    }

    pub fn remove_loops_and_multiedges(edges: Vec<(NodeId, DirectedWeightedEdge)>) -> Vec<(NodeId, DirectedWeightedEdge)>{
        let mut new_edges: Vec<(NodeId, DirectedWeightedEdge)> =  edges.into_iter()
        .filter(|(from, e)| *from != e.to )
        .collect(); //remove loops
        new_edges.sort();
        new_edges.dedup_by(|a, b| ->  bool {
            a.0 == b.0 && a.1.to == b.1.to
        }  ); //only keeps multiedge with smallest weigth
        new_edges
    }
}

impl From<Vec<(NodeId, DirectedWeightedEdge)>> for EdgeList {
    fn from(edge_list: Vec<(NodeId, DirectedWeightedEdge)>) -> Self {
        let n: usize = 1 + edge_list.iter().fold(0, |acc, (from, e)| std::cmp::max(acc, std::cmp::max(*from, e.head()))) as usize;
        let m: usize = edge_list.len();
        let mut deg: Vec<NodeId> = vec![0; n];
        for (from, e) in edge_list.iter() {
            deg[e.head() as usize] += 1;
            deg[*from as usize] += 1;
        }
        EdgeList{num_nodes: n, num_edges: m, degree: deg, edges: edge_list}
    }
}
impl Graph for EdgeList {
    fn num_nodes(&self) -> usize { self.num_nodes}
    fn num_arcs(&self) -> usize { self.num_edges}
    fn degree(&self, node: NodeId) -> usize {self.degree[node as usize] as usize}
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse() {
        // 0 --> {1,2,3,4}
        let edges: Vec<(NodeId, DirectedWeightedEdge)> = vec![
            (0, DirectedWeightedEdge::from_values(1, 1)),
            (0, DirectedWeightedEdge::from_values(2, 2)),
            (0, DirectedWeightedEdge::from_values(3, 3)),
            (0, DirectedWeightedEdge::from_values(4, 4)),
        ];
        let rev_edges: Vec<(NodeId, DirectedWeightedEdge)> = vec![
            (1, DirectedWeightedEdge::from_values(0, 1)),
            (2, DirectedWeightedEdge::from_values(0, 2)),
            (3, DirectedWeightedEdge::from_values(0, 3)),
            (4, DirectedWeightedEdge::from_values(0, 4)),
        ];
        let edge_list: EdgeList = edges.clone().into();
        let rev_edge_list: EdgeList = edge_list.reverse_edge_list();
        assert_eq!(edge_list.num_nodes(), rev_edge_list.num_nodes());
        assert_eq!(edge_list.num_arcs(), rev_edge_list.num_arcs());
        assert_eq!(rev_edges, rev_edge_list.edges);
        assert_eq!(rev_edge_list.degree(0), 0);
        for i in 1..5{
            assert_eq!(rev_edge_list.degree(i), 1);
        }
    }

    #[test]
    fn test_loop_multiedges() {
        let edges: Vec<(NodeId, DirectedWeightedEdge)> = vec![
            (0, DirectedWeightedEdge::from_values(0, 1)),
            (1, DirectedWeightedEdge::from_values(1, 1)),
            (0, DirectedWeightedEdge::from_values(2, 2)),
            (0, DirectedWeightedEdge::from_values(2, 3)),
            (0, DirectedWeightedEdge::from_values(2, 4)),
            (0, DirectedWeightedEdge::from_values(3, 5)),
            (0, DirectedWeightedEdge::from_values(3, 6)),
            (0, DirectedWeightedEdge::from_values(4, 7)),
        ];
        let red_edges: Vec<(NodeId, DirectedWeightedEdge)> = vec![
            (0, DirectedWeightedEdge::from_values(2, 2)),
            (0, DirectedWeightedEdge::from_values(3, 5)),
            (0, DirectedWeightedEdge::from_values(4, 7)),
        ];
        let reduced = EdgeList::remove_loops_and_multiedges(edges);
        assert_eq!(reduced, red_edges);
    }
}

