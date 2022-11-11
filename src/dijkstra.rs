use std::{collections::BinaryHeap, marker::PhantomData};

use crate::{graph::{definitions::*, nodes_edges::*, adjacency_list::*, edge_list::{EdgeList}, adjacency_array::AdjacencyArray}, datastructure::timestamped_vector::TimestampedVector};

const INVALID_PARENT: NodeId = NodeId::MAX;
pub const INFINITY: Distance = Distance::MAX;

/* potentials */
pub trait Potential {
    fn potential(&self, node: NodeId, target: NodeId) -> Weight;
}

pub struct ZeroPotential {}
impl ZeroPotential {
    pub fn new() -> ZeroPotential {ZeroPotential {}}
}
impl Potential for ZeroPotential {
    fn potential(&self, _node: NodeId, _target: NodeId) -> Weight { 0 }
}

pub struct L2Potential<'a> {
    coords: &'a Vec<(Coordinate, Coordinate)>,
}
impl L2Potential<'_> {
    pub fn new(_coords: &Vec<(Coordinate, Coordinate)>) -> L2Potential {
        L2Potential{coords: _coords}
    }
    fn l2dist(&self, v: NodeId, w: NodeId) -> Weight {
        let (xv, yv) = self.coords[v as usize];
        let (xw, yw) = self.coords[w as usize];
        (((xv - xw).pow(2) + (yv - yw).pow(2)) as f64).sqrt() as Weight
    }
}
impl Potential for L2Potential<'_> {
    fn potential(&self, node: NodeId, target: NodeId) -> Weight { self.l2dist(node, target) }
}

//deactivate landmarks, better heuristic for landmarks
pub struct AltPotential {
    dist_landmark: Vec<Vec<Distance>>,
}
impl AltPotential {
    pub fn new(dij: &mut Dijkstra<DirectedWeightedEdge, AdjacencyList>, landmarks: Vec<NodeId>) -> AltPotential {
        let mut d_landmarks: Vec<Vec<Distance>> = Vec::new();
        d_landmarks.reserve_exact(landmarks.len());
        for v in &landmarks {
            d_landmarks.push(dij.one_to_all(*v));
        }
        let mut d_landmarks_transposed: Vec<Vec<Distance>> = Vec::new();
        for j in 0..d_landmarks[0].len() {
            let mut vec: Vec<Distance> = Vec::new();
            for i in 0..landmarks.len() {
                vec.push(d_landmarks[i][j]);
            }
            d_landmarks_transposed.push(vec); 
        }
        AltPotential { dist_landmark: d_landmarks }
    }
    //use target to deactive some marks?
    fn alt_potential(&self, node: NodeId, _target: NodeId) -> Weight {
        let mut max_pot = 0; //memory layout better if index first by target? needs transpose -> test
        for i in 0..self.dist_landmark.len() {
            max_pot = std::cmp::max(max_pot, self.dist_landmark[i][node as usize]);
        }
        max_pot
        // *self.dist_landmark[node as usize].iter().max().unwrap() //not sure, more experiments
    }
}
impl Potential for AltPotential {
    fn potential(&self, node: NodeId, target: NodeId) -> Weight { self.alt_potential(node, target) }
}
/* potentials */

/* bidirectional selection rule */
#[derive(Clone, Copy, Debug)]
pub enum SearchDirection {
    Forward,
    Backward,
}
pub trait BidirektionalSelection {
    fn new() -> Self;
    fn decision<'a, Edge: Arc + Weighted, G: Graph + IncidentEdges<Edge>>(&mut self, forward_dij :&Dijkstra<'a, Edge, G>, backward_dij :&Dijkstra<'a, Edge, G>) -> SearchDirection {
        let a = forward_dij.pq.len();
        let b = backward_dij.pq.len();
        if a == 0 && b == 0 {
            panic!("both pq's are empty but search is not stoped");
        } else if a == 0 {
            return SearchDirection::Backward;
        } else if b == 0 {
            return SearchDirection::Forward;
        } else {
            return self.make_decision(forward_dij, backward_dij);
        }
    }
    fn make_decision<'a, Edge: Arc + Weighted, G: Graph + IncidentEdges<Edge>>(&mut self, forward_dij :&Dijkstra<'a, Edge, G>, backward_dij :&Dijkstra<'a, Edge, G>) -> SearchDirection;
}

pub struct Alternating {
    state: SearchDirection
}
impl Alternating{}

impl BidirektionalSelection for Alternating {
    fn new() -> Self {Alternating { state: (SearchDirection::Forward) }}
    fn make_decision<'a, Edge: Arc + Weighted, G: Graph + IncidentEdges<Edge>>(&mut self, _forward_dij :&Dijkstra<'a, Edge, G>, _backward_dij :&Dijkstra<'a, Edge, G>) -> SearchDirection {
        let decision = self.state;
        self.state = match self.state {
            SearchDirection::Forward => SearchDirection::Backward,
            SearchDirection::Backward => SearchDirection::Forward,
        };
        decision
    }
}
pub struct SmallerQueue {}
impl BidirektionalSelection for SmallerQueue {
    fn new() -> Self {SmallerQueue {  }}
    fn make_decision<'a, Edge: Arc + Weighted, G: Graph + IncidentEdges<Edge>>(&mut self, forward_dij :&Dijkstra<'a, Edge, G>, backward_dij :&Dijkstra<'a, Edge, G>) -> SearchDirection {
        if forward_dij.pq.len() <= backward_dij.pq.len() && forward_dij.pq.len() > 0 {
            return SearchDirection::Forward;
        } else {
            return SearchDirection::Backward;
        }
    }
}

pub struct SmallerQueueKey {}
impl BidirektionalSelection for SmallerQueueKey {
    fn new() -> Self {SmallerQueueKey {  }}
    fn make_decision<'a, Edge: Arc + Weighted, G: Graph + IncidentEdges<Edge>>(&mut self, forward_dij :&Dijkstra<'a, Edge, G>, backward_dij :&Dijkstra<'a, Edge, G>) -> SearchDirection {
        if forward_dij.pq_key() <= backward_dij.pq_key(){
            return SearchDirection::Forward;
        } else {
            return SearchDirection::Backward;
        }
    }
}

/* bidirectional selection rule */

/* bidirectional stop criterium */
pub trait BidirektionalStop {
    fn new(config: &DijkstraConfig) -> Self;
    fn finished<'a, Edge: Arc + Weighted, G: Graph + IncidentEdges<Edge>>(&mut self, forward_dij :&Dijkstra<'a, Edge, G>, backward_dij :&Dijkstra<'a, Edge, G>, tentative_distance: Distance, _settled: NodeId) -> bool;
}

pub struct SumQueueKeyStop {}
impl BidirektionalStop for SumQueueKeyStop {
    fn new(_config: &DijkstraConfig) -> Self {SumQueueKeyStop {}}
    fn finished<'a, Edge, G>(&mut self, forward_dij :&Dijkstra<'a, Edge, G>, backward_dij :&Dijkstra<'a, Edge, G>, tentative_distance: Distance, _settled: NodeId) -> bool 
    where Edge: Arc + Weighted, G: Graph + IncidentEdges<Edge>
    {
        return forward_dij.pq.is_empty() || backward_dij.pq.is_empty() ||
        (tentative_distance != INFINITY && tentative_distance <= forward_dij.progress + backward_dij.progress);
    }
}

pub struct MinQueueKeyStop {}
impl BidirektionalStop for MinQueueKeyStop {
    fn new(_config: &DijkstraConfig) -> Self {MinQueueKeyStop {}}
    fn finished<'a, Edge, G>(&mut self, forward_dij :&Dijkstra<'a, Edge, G>, backward_dij :&Dijkstra<'a, Edge, G>, tentative_distance: Distance, _settled: NodeId) -> bool 
    where Edge: Arc + Weighted, G: Graph + IncidentEdges<Edge>
    {   
        return forward_dij.pq.is_empty() || backward_dij.pq.is_empty() || 
        tentative_distance != INFINITY && tentative_distance <= std::cmp::min(forward_dij.pq_key(), backward_dij.pq_key());
    }
}

pub struct WitnessSearchStop {
    cap: Weight
}
impl BidirektionalStop for WitnessSearchStop {
    fn new(config: &DijkstraConfig) -> Self {
        let _cap: Weight;
        if config.cap.is_some() {
            _cap = config.cap.unwrap();
        } else {
            _cap = INFINITY;
        }
        WitnessSearchStop { cap: _cap }
    }
    fn finished<'a, Edge, G>(&mut self, fwd_dij :&Dijkstra<'a, Edge, G>, bwd_dij :&Dijkstra<'a, Edge, G>, _tentative_distance: Distance, settled: NodeId) -> bool 
    where Edge: Arc + Weighted, G: Graph + IncidentEdges<Edge>
    {   //search fronts reach cap or a node is settled from both sides -> stop from slides for bidirectional witness search
        return fwd_dij.pq.is_empty() || bwd_dij.pq.is_empty() || 
        fwd_dij.progress + bwd_dij.progress >= self.cap || 
        (fwd_dij.visited[settled as usize] == true && bwd_dij.visited[settled as usize] == true);
    }
}

pub struct CHSearchStop {}

impl BidirektionalStop for CHSearchStop {
    fn new(_config: &DijkstraConfig) -> Self { CHSearchStop {} }
    fn finished<'a, Edge, G>(&mut self, fwd_dij :&Dijkstra<'a, Edge, G>, bwd_dij :&Dijkstra<'a, Edge, G>, _tentative_distance: Distance, _settled: NodeId) -> bool 
    where Edge: Arc + Weighted, G: Graph + IncidentEdges<Edge>
    {   //search fronts reach cap or a node is settled from both sides -> stop from slides for bidirectional witness search
        return (fwd_dij.pq.is_empty() && bwd_dij.pq.is_empty()) || 
        _tentative_distance <= std::cmp::min(fwd_dij.progress, bwd_dij.progress);
    }
}

/* bidirectional stop criterium */

//information stored at one node in dijkstra

pub struct DijkstraConfig {
    pub start: NodeId,
    pub target: NodeId,
    pub cap: Option<Distance>, 
    pub max_steps: Option<usize>, 
    pub forbidden_node: Option<NodeId>
}

impl Default for DijkstraConfig {
    fn default() -> DijkstraConfig {
        DijkstraConfig { start: 0, target: 0, cap: None, max_steps: None, forbidden_node: None }
    }
}

pub struct DijkstraData {
    distances: TimestampedVector<Distance>,
    visited: TimestampedVector<bool>,
    parent: Vec<NodeId>, 
    pq: BinaryHeap<(Weight, NodeId)>, //max heap!
}

impl DijkstraData {
    pub fn new(num_nodes: usize) -> Self {
        DijkstraData { 
             distances: TimestampedVector::new(num_nodes, INFINITY),
             visited: TimestampedVector::new(num_nodes, false),
             parent: vec![INVALID_PARENT as NodeId; num_nodes],
             pq: BinaryHeap::new(),
        }
    }
}

pub struct Dijkstra<'a, Edge, G> where 
G: Graph + IncidentEdges<Edge>,
Edge: Arc + Weighted
{
    graph: &'a G,
    distances: &'a mut TimestampedVector<Distance>,
    visited: &'a mut TimestampedVector<bool>,
    parent: &'a mut Vec<NodeId>, 
    pq: &'a mut BinaryHeap<(Weight, NodeId)>, //max heap!
    progress: Weight, // = last pq key
    _phantom: PhantomData<Edge>,
}

impl<'a, Edge, G: Graph> Dijkstra<'a, Edge, G> where 
G: Graph + IncidentEdges<Edge>,
Edge: Arc + Weighted 
{
    pub fn new(_graph: &'a G, _data: &'a mut DijkstraData) -> Self {
        Dijkstra { 
            graph: _graph,
            distances: &mut _data.distances,
            visited: &mut _data.visited,
            parent: &mut _data.parent,
            pq: &mut _data.pq,
            progress: 0,
            _phantom: PhantomData }
    }

    fn init(&mut self) {
        self.distances.reset();
        self.visited.reset();
        self.pq.clear();
    }

    fn init_start_node(&mut self, start: NodeId) {
        self.distances[start as usize] = 0;
        self.pq.push((0, start));
        self.parent[start as usize] = start;
    }

    pub fn tentative_distance(&self, node: NodeId) -> Distance {
        self.distances[node as usize]
    }

    fn pq_key(&self) -> Weight {
        if let Some((key, _)) = self.pq.peek() {
            return -*key; //keys are negative because of max heap
        }
        return Weight::MAX;
    }
    
    //returns settled node or none
    pub fn generic_dijkstra_step<Pot: Potential>(&mut self, target: NodeId, pot: &Pot) -> Option<NodeId> {
        loop {
            if let Some((_, v_id)) = self.pq.pop() {
                let v = v_id as usize;
                if self.visited[v] { continue; } //until a node was settled
                self.visited.set(v, true);
                for e in self.graph.incident_edges(v_id) {
                    let to = e.head() as usize;
                    let d_new: Weight = self.distances[v] + e.weight();
                    if self.visited[to] { continue; }
                    if d_new < self.distances[to] {
                        self.distances.set(to, d_new);
                        let pq_key = -(d_new + pot.potential(e.head(), target));
                        self.pq.push((pq_key, e.head()));
                        self.parent[to] = v_id;
                    }
                }
                return Some(v_id);
            }
            return None;
        } 
    }

    fn generic_dijkstra<Pot: Potential>(&mut self, config: DijkstraConfig, pot: &Pot) {
        self.init();
        self.init_start_node(config.start);
        if let Some(u) = config.forbidden_node {
            self.visited[u as usize] = true; //deactivates node
        }
        let mut num_steps: usize = 0;
        while !self.pq.is_empty() {
            let settled_node: Option<NodeId> = self.generic_dijkstra_step(config.target, pot);
            if settled_node.is_none() { //pq only popped visited elements and pq is empty now
                break;
            }
            let v = settled_node.unwrap();
            num_steps += 1;
            if v == config.target {
                break;
            }
            let d = self.distances[v as usize]; // != INFINITY, since v got settled
            if config.cap.is_some() && d >= config.cap.unwrap() {
                break;
            }
            if config.max_steps.is_some() && num_steps >= config.max_steps.unwrap() {
                break;
            }
        }
    }

    pub fn generic_one_to_one<Pot: Potential>(&mut self, config: DijkstraConfig, pot: &Pot) -> Option<Distance> {
        let target = config.target as usize;
        self.generic_dijkstra(config, pot);
        if self.visited[target] {
            return Some(self.distances[target]);
        }
        return None;
    }

    pub fn one_to_one<Pot: Potential>(&mut self, start: NodeId, target: NodeId, pot: &Pot) -> Distance {
        let config: DijkstraConfig = DijkstraConfig { start: start, target: target, ..Default::default() };
        self.generic_dijkstra(config, pot);
        self.distances[target as usize]
    }

    pub fn one_to_all(&mut self, start: NodeId) -> Vec<Distance> {
        let dummy_target: NodeId = (self.graph.num_nodes() + 5) as NodeId; //algorithm does not stop earlier
        let config: DijkstraConfig = DijkstraConfig { start: start, target: dummy_target, ..Default::default() };
        self.generic_dijkstra(config, &ZeroPotential::new());
        self.distances.clone_data()
    }

    pub fn generic_bidrektional_dijkstra<SelectionRule: BidirektionalSelection, StopRule: BidirektionalStop>(
        config: DijkstraConfig,
        fwd_dij: &mut Dijkstra<'a, Edge, G>, 
        bwd_dij: &mut Dijkstra<'a, Edge, G>) -> Distance 
    {
        let start = config.start;
        let target = config.target;

        fwd_dij.init();
        bwd_dij.init();
        
        fwd_dij.init_start_node(start);
        bwd_dij.init_start_node(target);

        if config.forbidden_node.is_some() {
            let v: usize = config.forbidden_node.unwrap() as usize;
            fwd_dij.visited[v] = true;
            bwd_dij.visited[v] = true;
        }

        let forward_pot = ZeroPotential{};
        let backward_pot = ZeroPotential{};

        let mut selection_rule = SelectionRule::new();
        let mut stop_rule = StopRule::new(&config);
        
        let mut tentative_distance: Distance = INFINITY;
        let mut steps: usize = 0;
        let mut last_settled: NodeId = start; //only relevant if node is settled from both sides
        //have to finish search until both queues are empty for ch
        while !stop_rule.finished(fwd_dij, bwd_dij, tentative_distance, last_settled) {
        // while !fwd_dij.pq.is_empty() && !bwd_dij.pq.is_empty() && !stop_rule.finished(fwd_dij, bwd_dij, tentative_distance, last_settled) {
            let settled_node: Option<NodeId>;
            
            // selection: alternating, smaller pq, smaller pq key
            let dir = selection_rule.make_decision(&fwd_dij, &bwd_dij);
            match dir {
                SearchDirection::Forward => settled_node = fwd_dij.generic_dijkstra_step(target, &forward_pot),
                SearchDirection::Backward => settled_node = bwd_dij.generic_dijkstra_step(start, &backward_pot),
            }   
            if settled_node.is_none() { // pq only popped visited elements
                continue;
            }
            steps += 1;
            let node_id = settled_node.unwrap() as usize;
            last_settled = settled_node.unwrap();
            let fwd_dist = fwd_dij.distances[node_id];
            let bwd_dist = bwd_dij.distances[node_id];
            match dir {
                SearchDirection::Forward => fwd_dij.progress = fwd_dist,
                SearchDirection::Backward => bwd_dij.progress = bwd_dist,
            }   
            if std::cmp::max(fwd_dist, bwd_dist) < INFINITY { //both are not infinity
                tentative_distance = std::cmp::min(tentative_distance, fwd_dist + bwd_dist);
            }
            // println!("s {start}, t {target}, steps {steps}, settled {node_id}, d {tentative_distance}, df {}, db {}, dir {:?}, pq1 {}, pq2 {}", 
            // fwd_dist, bwd_dist, dir, fwd_dij.pq.len(), bwd_dij.pq.len());
            if config.max_steps.is_some() && steps >= config.max_steps.unwrap() {
                break;
            }
        }
        // println!("s {start}, t {target}, d {tentative_distance}, steps {steps}");
        return tentative_distance;
    }


    pub fn bidrektional_dijkstra<SelectionRule: BidirektionalSelection, StopRule: BidirektionalStop>(
        start: NodeId, target: NodeId, 
        fwd_dij: &mut Dijkstra<'a, Edge, G>, 
        bwd_dij: &mut Dijkstra<'a, Edge, G>) -> Distance 
    {
        let config: DijkstraConfig = DijkstraConfig { start: start, target: target, ..Default::default() };
        return Dijkstra::generic_bidrektional_dijkstra::<SelectionRule, StopRule>(config, fwd_dij, bwd_dij);
    }

}



pub trait OneToOne {
    fn one_to_one(&mut self, start: NodeId, target: NodeId) -> Distance;
}

pub struct DijkstraRunner<'a, Edge, G>  where  
    G: Graph + IncidentEdges<Edge>,
    Edge: Arc + Weighted,
{
    data: DijkstraData,
    graph: &'a G,
    _phantom: PhantomData<Edge>,
}

impl<'a, Edge: Arc + Weighted, G: Graph + IncidentEdges<Edge>> DijkstraRunner<'a, Edge, G> {
    pub fn new(graph: &'a G) -> Self {
        DijkstraRunner { data: DijkstraData::new(graph.num_nodes()), graph: &graph, _phantom: PhantomData }
    }
}
impl<'a, Edge: Arc + Weighted, G: Graph + IncidentEdges<Edge>> OneToOne for DijkstraRunner<'a, Edge, G>  {
    fn one_to_one(&mut self, start: NodeId, target: NodeId) -> Distance {
        let mut dij: Dijkstra<Edge, G> = Dijkstra::new(&self.graph, &mut self.data);
        return dij.one_to_one(start, target, &ZeroPotential{});
    }
}

pub struct BidirectionalRunner<SelectionRule: BidirektionalSelection, StopRule: BidirektionalStop>  where  
{
    data1: DijkstraData,
    data2: DijkstraData,
    fwd_graph: AdjacencyArray,
    bwd_graph: AdjacencyArray,
    _phantom1: PhantomData<SelectionRule>,
    _phantom2: PhantomData<StopRule>,
}

impl<SelectionRule: BidirektionalSelection, StopRule: BidirektionalStop> 
BidirectionalRunner<SelectionRule, StopRule> {
    pub fn new(edge_list: EdgeList) -> Self {
        BidirectionalRunner { data1: DijkstraData::new(edge_list.num_nodes()), data2: DijkstraData::new(edge_list.num_nodes()), fwd_graph: edge_list.clone().into(), bwd_graph: edge_list.reverse_edge_list().into(), _phantom1: PhantomData, _phantom2: PhantomData }
    }
}
impl<SelectionRule: BidirektionalSelection, StopRule: BidirektionalStop>  OneToOne 
for BidirectionalRunner<SelectionRule, StopRule>  {
    fn one_to_one(&mut self, start: NodeId, target: NodeId) -> Distance {
        let mut dij1: Dijkstra<DirectedWeightedEdge, AdjacencyArray> = Dijkstra::new(&self.fwd_graph, &mut self.data1);
        let mut dij2: Dijkstra<DirectedWeightedEdge, AdjacencyArray> = Dijkstra::new(&self.bwd_graph, &mut self.data2);
        return Dijkstra::bidrektional_dijkstra::<SelectionRule, StopRule>(start, target, &mut dij1, &mut dij2);
    }
}