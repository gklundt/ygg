use crate::uuid::guid_64::Guid;
use crate::graph_elements::edge::Edge;
use crate::graph_elements::node::Node;
use crate::metrics::uom;

use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use crate::metrics::formulas;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Graph {
    guid: Rc<Guid>,
    edges: HashMap<Rc<Guid>, Rc<Edge>>,
    nodes: HashMap<Rc<Guid>, Rc<Node>>,
    node_connections: HashMap<Rc<Guid>, HashMap<Rc<Guid>, Rc<Guid>>>,
    // source, (target, edge)[]
    edge_connections: HashMap<Rc<Guid>, (Rc<Guid>, Rc<Guid>)>,
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
                self.nodes.get(&(connection.1).0).unwrap(),
                self.nodes.get(&(connection.1).1).unwrap()
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
        self.sub_graphs.insert(sub_graph.get_guid(), sub_graph);
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

    pub fn get_edge_connections(&mut self) -> &HashMap<Rc<Guid>, (Rc<Guid>, Rc<Guid>)> {
        &self.edge_connections
    }

    pub fn get_degree(&self, node: Rc<Guid>) -> usize {
        match self.node_connections.get(&node) {
            Some(conn) => conn.len(),
            None => 0,
        }
    }

    pub fn get_guid(&self) -> Rc<Guid> {
        self.guid.clone()
    }

    pub fn add_node(&mut self, node: Rc<Node>) {
        let guid: Rc<Guid> = Rc::clone(&node.get_guid());
        self.nodes.insert(guid, node);
    }

    pub fn add_connected_node_guids(&mut self, connection: (Rc<Guid>, Rc<Guid>)) {
        if let (Some(source_node), Some(target_node)) = (self.get_node(connection.0), self.get_node(connection.1)) {
            self.add_connected_nodes((source_node, target_node))
        }
    }

    pub fn get_tree_for_node(&self, node_guid: Rc<Guid>) -> Option<VecDeque<Rc<Guid>>> {
        let mut tree = VecDeque::new();
        tree.push_back(node_guid.clone());
        self.get_tree_for_node_(tree)
    }

    fn get_tree_for_node_(&self, mut tree: VecDeque<Rc<Guid>>) -> Option<VecDeque<Rc<Guid>>> {
        if let Some(last_node) = tree.back() {
            if let Some(connections) = self.node_connections.get(last_node) {
                let mut ret: Option<VecDeque<Rc<Guid>>> = None;
                for connection in connections {
                    if let false = tree.clone().contains(&connection.0.clone()) {
                        tree.push_back(connection.0.clone());
                        if let Some(sub_tree) = self.get_tree_for_node_(tree.clone()) {
                            ret = Some(sub_tree);
                        } else { ret = None }
                    } else { ret = None }
                }
                ret
            } else { Some(tree) }
        } else { Some(tree) }
    }

    pub fn get_path_for_node(&self, node_guid: Rc<Guid>) -> Option<VecDeque<Rc<Guid>>> {
        let degree = self.get_degree(node_guid.clone()) as u32;
        let degree_test = degree == 1 || degree == 2;
        match degree_test {
            false => None,
            true => {
                let mut path = VecDeque::new();

                let mut connections = Vec::new();
                if let Some(map) = self.node_connections.get(&node_guid.clone()) {
                    for node_connection in map {
                        connections.push(node_connection.0.clone());
                    }
                }

                path.push_front(node_guid.clone());
                path.push_back(connections[0].clone());
                if let true = degree == 2 { path.push_front(connections[1].clone()) };
                Some(self.get_path_for_node_(path))


//                let test_path = self.get_path_for_node_(path.clone()).clone();
//
//                let mut ret: Option<VecDeque<Rc<Guid>>> = Some(self.get_path_for_node_(path));
//                for item in test_path {
//                    let deg = self.get_degree(item.clone()) as u32;
//                    if let true = deg > 2 {
//                        ret = None;
//                        break;
//                    }
//                }
//                ret
            }
        }
    }


    pub fn add_connected_nodes(&mut self, connection: (Rc<Node>, Rc<Node>)) {
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

    pub fn remove_node_connection(&mut self, connection: (Rc<Node>, Rc<Node>)) {
        let given: (Rc<Node>, Rc<Node>) = (connection.0.clone(), connection.1.clone());
        let reverse: (Rc<Node>, Rc<Node>) = (connection.1.clone(), connection.0.clone());
        self.remove_node_connection_(&given);
        self.remove_node_connection_(&reverse);
    }

    pub fn remove_edge_connection(&mut self, connection: Rc<Edge>) {
        if let Some(edge) = self.edge_connections.get(&connection.get_guid()) {
            if let Some(l) = self.nodes.get(&edge.0) {
                if let Some(r) = self.nodes.get(&edge.1) {
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
//for j in { 0..h.len() } {
                self.add_connected_nodes((Rc::clone(&h[i]), Rc::clone(&h[j])));
            }
        }
        println!("Edges Created: {}.", self.edges.len());
    }
}

impl Graph {
    fn get_path_for_node_(&self, mut path: VecDeque<Rc<Guid>>) -> VecDeque<Rc<Guid>> {
        if let Some(left_node) = path.front() {
            if let Some(map) = self.node_connections.get(left_node) {
                for node_connection in map {
                    if let false = path.contains(node_connection.0) {
                        path.push_front(node_connection.0.clone());
                        path = self.get_path_for_node_(path);
                    }
                }
            }
        }

        if let Some(right_node) = path.back() {
            if let Some(map) = self.node_connections.get(right_node) {
                for node_connection in map {
                    if let false = path.contains(node_connection.0) {
                        path.push_back(node_connection.0.clone());
                        path = self.get_path_for_node_(path);
                    }
                }
            }
        }

        path
    }

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