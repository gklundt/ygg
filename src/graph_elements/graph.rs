use crate::uuid::guid_64::Guid;
use crate::graph_elements::edge::Edge;
use crate::graph_elements::node::Node;
use crate::metrics::formulas;
use crate::metrics::uom;

use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Graph {
    edges: HashMap<Rc<Guid>, Rc<Edge>>,
    nodes: HashMap<Rc<Guid>, Rc<Node>>,
    node_connections: HashMap<Rc<Guid>, HashMap<Rc<Guid>, Rc<Guid>>>,
    edge_connections: HashMap<Rc<Guid>, (Rc<Guid>, Rc<Guid>)>,
    sub_graphs: Vec<Box<Graph>>,
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

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::from("");
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
                "\tEdge:{} connects nodes:({}) -> ({})\n",
                connection.0,
                (connection.1).0,
                (connection.1).1
            );
            s.push_str(&ns);
        }
        write!(f, "{}", s)
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
            nodes: HashMap::new(),
            node_connections: HashMap::new(),
            edge_connections: HashMap::new(),
            sub_graphs: Vec::new(),
        }
    }

    pub fn get_nodes(&self) -> &HashMap<Rc<Guid>, Rc<Node>> {
        &self.nodes
    }

    pub fn add_node(&mut self, node: Rc<Node>) {
        let guid: Rc<Guid> = Rc::clone(&node.get_guid());
        self.nodes.insert(guid, node);
    }

    pub fn remove_node_connection(&mut self, connection: (Rc<Node>, Rc<Node>)) {
        let given: (Rc<Node>, Rc<Node>) = (connection.0.clone(), connection.1.clone());
        let reverse: (Rc<Node>, Rc<Node>) = (connection.1.clone(), connection.0.clone());
        self.remove_node_connection_(&given);
        self.remove_node_connection_(&reverse);
    }

    pub fn add_connection(&mut self, connection: (Rc<Node>, Rc<Node>)) {
        if let true = Rc::ptr_eq(&connection.0, &connection.1) {
            return;
        };

        let left_edge_state: EdgeState = self.get_edge_state(connection.0.get_guid(), connection.1.get_guid());
        let right_edge_state: EdgeState = self.get_edge_state(connection.1.get_guid(), connection.0.get_guid());
        let action: ActionState = self.get_action_state(left_edge_state, right_edge_state);

        match action {
            ActionState::CheckEdgeEquality(leg, reg) => match Rc::ptr_eq(&leg, &reg) {
                true => return,
                false => self.confirm_matching_edge(leg, reg, connection.0.get_guid(), connection.1.get_guid()),
            },
            ActionState::AssociateRight(leg) => self.associate_node(connection.1.get_guid(), leg, connection.0.get_guid()),
            ActionState::AssociateLeft(reg) => self.associate_node(connection.0.get_guid(), reg, connection.1.get_guid()),
            ActionState::AddRight(leg) => self.add_associate_node(connection.1.get_guid(), leg, connection.0.get_guid()),
            ActionState::AddLeft(reg) => self.add_associate_node(connection.0.get_guid(), reg, connection.1.get_guid()),
            ActionState::AssociateBoth => {
                let new_edge: Rc<Edge> = self.create_edge(connection.0.get_guid(), connection.1.get_guid());
                self.associate_node(connection.0.get_guid(), new_edge.get_guid(), connection.1.get_guid());
                self.associate_node(connection.1.get_guid(), new_edge.get_guid(), connection.0.get_guid());
            }
            ActionState::AddRightAssociateLeft => {
                let new_edge: Rc<Edge> = self.create_edge(connection.0.get_guid(), connection.1.get_guid());
                self.associate_node(connection.0.get_guid(), new_edge.get_guid(), connection.1.get_guid());
                self.add_associate_node(connection.1.get_guid(), new_edge.get_guid(), connection.0.get_guid());
            }
            ActionState::AddLeftAssociateRight => {
                let new_edge: Rc<Edge> = self.create_edge(connection.0.get_guid(), connection.1.get_guid());
                self.add_associate_node(connection.0.get_guid(), new_edge.get_guid(), connection.1.get_guid());
                self.associate_node(connection.1.get_guid(), new_edge.get_guid(), connection.0.get_guid());
            }
            ActionState::AddBoth => {
                let new_edge: Rc<Edge> = self.create_edge(connection.0.get_guid(), connection.1.get_guid());
                self.add_associate_node(connection.1.get_guid(), new_edge.get_guid(), connection.0.get_guid());
                self.add_associate_node(connection.0.get_guid(), new_edge.get_guid(), connection.1.get_guid());
            }
        }
    }

// pub fn remove_edge_connection(&mut self, connection: Rc<Edge>) {
//     let exists = match self.edge_connections.get_mut(&connection.get_guid()) {
//         Some(x) => Some(x),
//         None => None,
//     };
//     let left = match exists {
//         Some(x) => self.nodes.get(&x.0),
//         None => None,
//     };

//     let right = match exists {
//         Some(x) => self.nodes.get(&x.1),
//         None => None,
//     };
// }

    pub fn connect_all_nodes(&mut self) {
        let mut h: Vec<Rc<Node>> = Vec::new();
        for src in self.nodes.values() {
            h.push(Rc::clone(src));
        }
        println!("Original Vector Length: {} items.", h.len());
        for i in { 0..h.len() } {
            for j in { i + 1..h.len() } {
                //for j in { 0..h.len() } {
                self.add_connection((Rc::clone(&h[i]), Rc::clone(&h[j])));
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

    fn add_edge(&mut self, edge: Rc<Edge>) {
        let guid: Rc<Guid> = Rc::clone(&edge.get_guid());
        self.edges.insert(guid, edge);
    }

    fn get_edge_state(&self, source_node: Rc<Guid>, target_node: Rc<Guid>) -> EdgeState {
        match self.node_connections.get(&source_node) {
            Some(assoc_hm) => match assoc_hm.get(&target_node) {
                Some(assoc_edge) => EdgeState::Associated(Rc::clone(assoc_edge)),
                None => EdgeState::NotAssociated,
            },
            None => EdgeState::DoesNotExist,
        }
    }

    fn get_action_state(&self, left_edge_state: EdgeState, right_edge_state: EdgeState) -> ActionState {
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

    fn confirm_matching_edge(&mut self, left_edge_guid: Rc<Guid>, right_edge_guid: Rc<Guid>, source_node: Rc<Guid>, target_node: Rc<Guid>) {
        match self.edges.remove(&right_edge_guid) { _ => (), }
        match self.edge_connections.remove(&right_edge_guid) { _ => (), }
        if let Some(x) = self.node_connections.get_mut(&target_node) {
            match x.insert(Rc::clone(&source_node), Rc::clone(&left_edge_guid)) { _ => (), }
        }
    }

    fn create_edge(&mut self, source_node: Rc<Guid>, target_node: Rc<Guid>) -> Rc<Edge> {
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
        self.add_edge(Rc::clone(&new_edge));
        self.edge_connections.insert(
            new_edge.get_guid(),
            (source_node, target_node),
        );
        new_edge.clone()
    }

    fn associate_node(&mut self, source_node: Rc<Guid>, edge: Rc<Guid>, target_node: Rc<Guid>) {
        if let Some(map) = self.node_connections.get_mut(&source_node) { map.insert(target_node, edge); }
    }

    fn add_associate_node(&mut self, source_node: Rc<Guid>, edge: Rc<Guid>, target_node: Rc<Guid>) {
        let mut r: HashMap<Rc<Guid>, Rc<Guid>> = HashMap::new();
        r.insert(target_node, edge);
        self.node_connections.insert(source_node, r);
    }
}