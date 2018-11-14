use crate::graph_elements::edge::Edge;
use crate::graph_elements::node::Node;
use crate::metrics::formulas;
use crate::metrics::uom;
use crate::uuid::guid_64::Guid;
use crate::graph_elements::node_pair::NodePair;

use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;


#[derive(Debug)]
pub struct Graph {
    guid: Rc<Guid>,
    edges: HashMap<Rc<Guid>, Rc<Edge>>,
    nodes: HashMap<Rc<Guid>, Rc<Node>>,
    node_connections: HashMap<Rc<Guid>, HashMap<Rc<Guid>, Rc<Guid>>>,
    // source, (target, edge)[]
    edge_connections: HashMap<Rc<Guid>, Rc<NodePair>>,
    // edge, (source, target)
    sub_graphs: HashMap<Rc<Guid>, Rc<Graph>>,
    named_graphs: HashMap<Rc<String>, Rc<Guid>>,
    named_nodes: HashMap<Rc<String>, Rc<Guid>>,
}


#[derive(Debug)]
enum EdgeState {
    Associated(Rc<Guid>),
    NotAssociated,
    DoesNotExist,
}

#[derive(Debug)]
enum ActionState {
    CheckEdgeEquality(Rc<Guid>, Rc<Guid>),
    AssociateRight(Rc<Guid>),
    AssociateLeft(Rc<Guid>),
    AddRight(Rc<Guid>),
    AddLeft(Rc<Guid>),
    AssociateBoth,
    AddRightAssociateLeft,
    AddLeftAssociateRight,
    AddBoth,
}


impl Graph {
    pub fn get_tree_for_node(&self, node_guid: Rc<Guid>) -> Option<Vec<Rc<Guid>>> {
        let mut params = (Vec::new(), Vec::new(), false);
        println!("Sending in {}", self.nodes.get(&node_guid.clone()).unwrap().get_name());
        params = self.get_tree_for_node_(node_guid, params);
        match params.2 {
            true => None,
            false => Some(params.0.clone())
        }
    }

    fn get_tree_for_node_(&self, current_node: Rc<Guid>, mut params: (Vec<Rc<Guid>>, Vec<Rc<NodePair>>, bool)) -> (Vec<Rc<Guid>>, Vec<Rc<NodePair>>, bool) {
        params.0.push(current_node.clone());
        for edge_connection in &self.edge_connections {
            if let true = params.1.clone().contains(edge_connection.1) { continue; }
            if let true = edge_connection.1.contains(current_node.clone()) {
                if let Some(peer) = edge_connection.1.get_peer(current_node.clone()) {
                    params.1.push(edge_connection.1.clone());
                    params = self.get_tree_for_node_(peer, params);
                }
            }
        }
        params
    }
}


impl Graph {
    pub fn get_ordered_path_for_node(&self, node_guid: Rc<Guid>) -> Option<Vec<Rc<Guid>>> {
        let mut find = node_guid.clone();
        if let Some(nodes) = self.get_path_for_node(node_guid.clone()) {
            for node in nodes {
                if let true = self.get_degree_of_node(node.clone()) == 1 {
                    find = node;
                    break;
                }
            }
        }
        self.get_path_for_node(find)
    }

    pub fn get_path_for_node(&self, node_guid: Rc<Guid>) -> Option<Vec<Rc<Guid>>> {
        let mut path = Vec::new();
        path.push(node_guid.clone());

        let ret = self.get_path_for_node_(path, 0);
        match ret.1 {
            true => None, // a cycle was found
            false => match ret.2 {
                false => None, // not a path, vertex found with degree > 2
                true => Some(ret.0), // yay, a path
            },
        }
    }

    fn get_path_for_node_(&self, mut path: Vec<Rc<Guid>>, pos: usize) -> (Vec<Rc<Guid>>, bool, bool) {
        let mut cycle = false;
        let mut is_path = true;
        if let (Some(prev_node), Some(last_node)) =
        (path.clone().get(pos), path.clone().last()) {
            if let Some(connections) = self.node_connections.get(last_node) {
                if let true = connections.len() > 2 { is_path = false; }
                let my_pos = path.clone().len() - 1;
                for connection in connections {
                    if let true = Rc::ptr_eq(&connection.0.clone(), prev_node) { continue; }
                    if let false = path.clone().contains(&connection.0.clone()) {
                        path.push(connection.0.clone());
                        path = self.get_path_for_node_(path, my_pos).0;
                    } else { cycle = true; }
                }
            }
        }
        (path, cycle, is_path)
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::from(format!("Graph Guid: {}\n", self.guid.to_string()));
        s.push_str("Nodes:\n");
        for node in &self.nodes {
            let ns = format!("\tNode:{}\n", node.1);
            s.push_str(&ns);
        }

        s.push_str("Edges:\n");
        for edge in &self.edges {
            let ns: String = format!("\tEdge:{}\n", edge.1);
            s.push_str(&ns);
        }

        s.push_str("Connections:\n");

        for connection in &self.edge_connections {
            let ns: String = format!(
                "\tEdge:{} connects \n\t\tnode: {} \n\t\tto:   {}\n",
                self.edges.get(&(connection.0.clone())).unwrap(),
                self.nodes.get(&(connection.1).get_left()).unwrap(),
                self.nodes.get(&(connection.1).get_right()).unwrap()
            );
            s.push_str(&ns);
        }
        write!(f, "{}", s)
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            guid: Guid::new(),
            edges: HashMap::new(),
            nodes: HashMap::new(),
            node_connections: HashMap::new(),
            edge_connections: HashMap::new(),
            sub_graphs: HashMap::new(),
            named_graphs: HashMap::new(),
            named_nodes: HashMap::new(),
        }
    }

    pub fn replicate_all(&self) -> Self {
        let mut g = Graph::new();
        for edge in &self.edges { g.edges.insert(edge.0.clone(), edge.1.clone()); }
        for node in &self.nodes { g.nodes.insert(node.0.clone(), node.1.clone()); }
        for nc in &self.node_connections { g.node_connections.insert(nc.0.clone(), nc.1.clone()); }
        for ec in &self.edge_connections { g.edge_connections.insert(ec.0.clone(), ec.1.clone()); }
        for nn in &self.named_nodes { g.named_nodes.insert(nn.0.clone(), nn.1.clone()); }
        g
    }

    pub fn replicate_nodes(&self) -> Self {
        let mut g = Graph::new();
        for node in &self.nodes { g.nodes.insert(node.0.clone(), node.1.clone()); }
        g
    }

    pub fn add_sub_graph(&mut self, sub_graph: Rc<Graph>) {
        self.sub_graphs.insert(sub_graph.get_graph_guid(), sub_graph);
    }

    pub fn get_sub_graphs(&self) -> &HashMap<Rc<Guid>, Rc<Graph>> {
        &self.sub_graphs
    }

    pub fn get_graph(&self, graph_guid: Rc<Guid>) -> Option<&Self> {
        match Rc::ptr_eq(&self.guid, &graph_guid) {
            true => Some(self),
            false => {
                let mut ret: Option<&Graph> = None;
                for g in &self.sub_graphs {
                    ret = match g.1.get_graph(graph_guid.clone()) {
                        Some(g) => Some(g),
                        None => None,
                    };
                    if let Some(_) = ret { break; }
                }
                ret
            }
        }
    }

    pub fn get_named_node(&self, name: Rc<String>) -> Option<Rc<Node>> {
        match self.named_nodes.get(&name) {
            Some(guid) => self.get_node(guid.clone()),
            _ => None,
        }
    }

    pub fn get_node(&self, guid: Rc<Guid>) -> Option<Rc<Node>> {
        match self.nodes.get(&guid.clone()) {
            Some(node) => Some(node.clone()),
            _ => None,
        }
    }

    pub fn get_nodes(&self) -> &HashMap<Rc<Guid>, Rc<Node>> {
        &self.nodes
    }

    pub fn get_edges(&mut self) -> &HashMap<Rc<Guid>, Rc<Edge>> {
        &self.edges
    }

    pub fn get_edge_connections(&mut self) -> &HashMap<Rc<Guid>, Rc<NodePair>> {
        &self.edge_connections
    }

    pub fn get_degree_of_node(&self, node: Rc<Guid>) -> usize {
        match self.node_connections.get(&node) {
            Some(conn) => conn.len(),
            None => 0,
        }
    }

    pub fn get_graph_guid(&self) -> Rc<Guid> {
        self.guid.clone()
    }

    pub fn add_node(&mut self, node: Rc<Node>) {
        let guid: Rc<Guid> = Rc::clone(&node.get_guid());
        self.nodes.insert(guid, node);
    }

    pub fn add_connected_nodes_by_guid(&mut self, connection: Rc<NodePair>) {
        if let (Some(source_node), Some(target_node)) = (self.get_node(connection.get_left()), self.get_node(connection.get_right())) {
            self.add_connected_nodes((source_node, target_node))
        }
    }

    pub fn add_connected_nodes(&mut self, connection: (Rc<Node>, Rc<Node>)) {
        if let true = Rc::ptr_eq(&connection.0, &connection.1) {
            return;
        };

        let left_edge_state: EdgeState = self.get_edge_state_(connection.0.get_guid(), connection.1.get_guid());
        let right_edge_state: EdgeState = self.get_edge_state_(connection.1.get_guid(), connection.0.get_guid());
        let action: ActionState = self.get_action_state_(left_edge_state, right_edge_state);

        match action {
            ActionState::CheckEdgeEquality(leg, reg) => match Rc::ptr_eq(&leg, &reg) {
                true => return,
                false => self.confirm_matching_edge_(leg, reg, connection.0.get_guid(), connection.1.get_guid()),
            },
            ActionState::AssociateRight(leg) => self.associate_node_(connection.1.get_guid(), leg, connection.0.get_guid()),
            ActionState::AssociateLeft(reg) => self.associate_node_(connection.0.get_guid(), reg, connection.1.get_guid()),
            ActionState::AddRight(leg) => self.add_associate_node_(connection.1.get_guid(), leg, connection.0.get_guid()),
            ActionState::AddLeft(reg) => self.add_associate_node_(connection.0.get_guid(), reg, connection.1.get_guid()),
            ActionState::AssociateBoth => {
                let new_edge: Rc<Edge> = self.create_edge_(connection.0.get_guid(), connection.1.get_guid());
                self.associate_node_(connection.0.get_guid(), new_edge.get_guid(), connection.1.get_guid());
                self.associate_node_(connection.1.get_guid(), new_edge.get_guid(), connection.0.get_guid());
            }
            ActionState::AddRightAssociateLeft => {
                let new_edge: Rc<Edge> = self.create_edge_(connection.0.get_guid(), connection.1.get_guid());
                self.associate_node_(connection.0.get_guid(), new_edge.get_guid(), connection.1.get_guid());
                self.add_associate_node_(connection.1.get_guid(), new_edge.get_guid(), connection.0.get_guid());
            }
            ActionState::AddLeftAssociateRight => {
                let new_edge: Rc<Edge> = self.create_edge_(connection.0.get_guid(), connection.1.get_guid());
                self.add_associate_node_(connection.0.get_guid(), new_edge.get_guid(), connection.1.get_guid());
                self.associate_node_(connection.1.get_guid(), new_edge.get_guid(), connection.0.get_guid());
            }
            ActionState::AddBoth => {
                let new_edge: Rc<Edge> = self.create_edge_(connection.0.get_guid(), connection.1.get_guid());
                self.add_associate_node_(connection.1.get_guid(), new_edge.get_guid(), connection.0.get_guid());
                self.add_associate_node_(connection.0.get_guid(), new_edge.get_guid(), connection.1.get_guid());
            }
        }
    }

    pub fn remove_node_connection(&mut self, connection: (Rc<Node>, Rc<Node>)) {
        let given: (Rc<Node>, Rc<Node>) = (connection.0.clone(), connection.1.clone());
        let reverse: (Rc<Node>, Rc<Node>) = (connection.1.clone(), connection.0.clone());
        self.remove_node_connection_(&given);
        self.remove_node_connection_(&reverse);
    }

    pub fn remove_edge_connection(&mut self, connection: Rc<Edge>) {
        if let Some(edge) = self.edge_connections.get(&connection.get_guid()) {
            if let Some(l) = self.nodes.get(&edge.get_pair().0) {
                if let Some(r) = self.nodes.get(&edge.get_pair().1) {
                    self.remove_node_connection((l.clone(), r.clone()));
                }
            }
        }
    }

    pub fn remove_all_edges(&mut self) {
        let p = self.edges.clone();
        for e in p {
            self.remove_edge_connection(e.1);
        }
    }

    pub fn connect_all_nodes(&mut self) {
        let mut h: Vec<Rc<Node>> = Vec::new();
        for src in self.nodes.values() {
            h.push(Rc::clone(src));
        }
        println!("Original Vector Length: {} items.", h.len());
        for i in { 0..h.len() } {
            for j in { i + 1..h.len() } {
                self.add_connected_nodes((h[i].clone(), h[j].clone()));
            }
        }
        println!("Edges Created: {}.", self.edges.len());
    }
}

impl Graph {
    fn remove_node_connection_(&mut self, connection: &(Rc<Node>, Rc<Node>)) {
        if let Some(map) = self.node_connections.get_mut(&connection.0.get_guid()) {
            if let Some(edge_guid) = map.remove(&connection.1.get_guid()) {
                if let Some(edge_guid) = self.edge_connections.remove_entry(&edge_guid) {
                    self.edges.remove(&edge_guid.0);
                }
            }
        }
    }

    fn add_edge_(&mut self, edge: Rc<Edge>) {
        let guid: Rc<Guid> = Rc::clone(&edge.get_guid());
        self.edges.insert(guid, edge);
    }

    fn get_edge_state_(&self, source_node: Rc<Guid>, target_node: Rc<Guid>) -> EdgeState {
        match self.node_connections.get(&source_node) {
            Some(assoc_hm) => match assoc_hm.get(&target_node) {
                Some(assoc_edge) => EdgeState::Associated(Rc::clone(assoc_edge)),
                None => EdgeState::NotAssociated,
            },
            None => EdgeState::DoesNotExist,
        }
    }

    fn get_action_state_(&self, left_edge_state: EdgeState, right_edge_state: EdgeState) -> ActionState {
        match (left_edge_state, right_edge_state) {
            (EdgeState::Associated(leg), EdgeState::NotAssociated) => { ActionState::AssociateRight(leg) }
            (EdgeState::Associated(leg), EdgeState::DoesNotExist) => { ActionState::AddRight(leg) }
            (EdgeState::Associated(leg), EdgeState::Associated(reg)) => { ActionState::CheckEdgeEquality(leg, reg) }
            (EdgeState::NotAssociated, EdgeState::Associated(reg)) => { ActionState::AssociateLeft(reg) }
            (EdgeState::NotAssociated, EdgeState::NotAssociated) => { ActionState::AssociateBoth }
            (EdgeState::NotAssociated, EdgeState::DoesNotExist) => { ActionState::AddRightAssociateLeft }
            (EdgeState::DoesNotExist, EdgeState::Associated(reg)) => { ActionState::AddLeft(reg) }
            (EdgeState::DoesNotExist, EdgeState::NotAssociated) => { ActionState::AddLeftAssociateRight }
            (EdgeState::DoesNotExist, EdgeState::DoesNotExist) => ActionState::AddBoth,
        }
    }

    fn confirm_matching_edge_(&mut self, left_edge_guid: Rc<Guid>, right_edge_guid: Rc<Guid>, source_node: Rc<Guid>, target_node: Rc<Guid>) {
        match self.edges.remove(&right_edge_guid) { _ => (), }
        match self.edge_connections.remove(&right_edge_guid) { _ => (), }
        if let Some(x) = self.node_connections.get_mut(&target_node) {
            match x.insert(Rc::clone(&source_node), Rc::clone(&left_edge_guid)) { _ => (), }
        }
    }

    fn create_edge_(&mut self, source_node: Rc<Guid>, target_node: Rc<Guid>) -> Rc<Edge> {
        let mut ln: &uom::PositionKind = &uom::PositionKind::Unknown;
        if let Some(node) = &self.nodes.get(&source_node) { ln = node.get_position() };

        let mut rn: &uom::PositionKind = &uom::PositionKind::Unknown;
        if let Some(node) = &self.nodes.get(&target_node) { rn = node.get_position() };

        let distance: uom::DistanceKind = formulas::distance_between_two_points(ln, rn);
        let edge_distance = match distance {
            uom::DistanceKind::Unknown => None,
            _ => Some(distance),
        };

        let new_edge: Rc<Edge> = Rc::new(Edge::new(edge_distance));
        self.add_edge_(Rc::clone(&new_edge));
        self.edge_connections.insert(
            new_edge.get_guid(),
            Rc::new(NodePair::new((source_node, target_node))),
        );
        new_edge.clone()
    }

    fn associate_node_(&mut self, source_node: Rc<Guid>, edge: Rc<Guid>, target_node: Rc<Guid>) {
        if let Some(map) = self.node_connections.get_mut(&source_node) { map.insert(target_node, edge); }
    }

    fn add_associate_node_(&mut self, source_node: Rc<Guid>, edge: Rc<Guid>, target_node: Rc<Guid>) {
        let mut r: HashMap<Rc<Guid>, Rc<Guid>> = HashMap::new();
        r.insert(target_node, edge);
        self.node_connections.insert(source_node, r);
    }
}