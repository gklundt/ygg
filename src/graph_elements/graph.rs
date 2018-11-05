use super::super::uuid::Guid;
use super::edge::Edge;
use super::node::Node;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

enum EdgeState {
    Good,
    UpdateEdges(Rc<Guid>, Rc<Guid>),
    AddRight(Rc<Guid>),
    AddLeft(Rc<Guid>),
    AddBoth,
}

#[derive(Debug)]
pub struct Graph {
    edges: HashMap<Rc<Guid>, Rc<Edge>>,
    nodes: HashMap<Rc<Guid>, Rc<Node>>,
    // node and a collection of node key/edge value
    node_connections: HashMap<Rc<Guid>, HashMap<Rc<Guid>, Rc<Guid>>>,
    edge_connections: HashMap<Rc<Guid>, (Rc<Guid>, Rc<Guid>)>,
    subgraphs: Vec<Box<Graph>>,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("");
        s.push_str("Nodes:\n");
        for node in &self.nodes {
            let ns = format!("\tNode:{}\n", node.1);
            s.push_str(&ns);
        }
        s.push_str("Edges:\n");
        for edge in &self.edges {
            let ns = format!("\tEdge:{}\n", edge.1);
            s.push_str(&ns);
        }
        s.push_str("Connections:\n");

        for connection in &self.edge_connections {
            let ns = format!(
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
    pub fn new() -> Graph {
        Graph {
            edges: HashMap::new(),
            nodes: HashMap::new(),
            node_connections: HashMap::new(),
            edge_connections: HashMap::new(),
            subgraphs: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Rc<Node>) {
        let guid = Rc::clone(&node.get_guid());
        self.nodes.insert(guid, node);
    }

    fn add_edge(&mut self, edge: Rc<Edge>) {
        let guid = Rc::clone(&edge.get_guid());
        self.edges.insert(guid, edge);
    }

    fn is_node_in_connections(&self, node: Rc<Node>) -> Option<&HashMap<Rc<Guid>, Rc<Guid>>> {
        match self.node_connections.get(&node.get_guid()) {
            Some(x) => Some(x),
            None => None,
        }
    }

    fn is_node_attached_in_connection(
        &self,
        connection: (&Rc<Node>, &Rc<Node>),
    ) -> Option<&Rc<Guid>> {
        match self.is_node_in_connections(Rc::clone(&connection.0)) {
            Some(x) => match x.get(&connection.1.get_guid()) {
                Some(y) => Some(y),
                None => None,
            },
            None => None,
        }
    }

    pub fn add_connection(&mut self, connection: (Rc<Node>, Rc<Node>)) {
        // No Loops!
        match Rc::ptr_eq(&connection.0, &connection.1) {
            true => return,
            false => (),
        };

        let reverse = (&connection.1, &connection.0);
        let given_pair = (&connection.0, &connection.1);

        let left = match self.is_node_attached_in_connection(given_pair) {
            Some(x) => Some(x),
            None => None,
        };

        let right = match self.is_node_attached_in_connection(reverse) {
            Some(x) => Some(x),
            None => None,
        };

        let action = match left {
            Some(x) => match right {
                Some(y) => match x == y {
                    true => EdgeState::Good, // this is good
                    false => EdgeState::UpdateEdges(Rc::clone(x), Rc::clone(y)) // this is bad .. nodes are connected with different edges,
                },
                None => EdgeState::AddRight(Rc::clone(x)) // left side exists, but right doesn't,
            },
            None => {
                match right {
                    Some(x) => EdgeState::AddLeft(Rc::clone(x)), // add correspoonding left edge with provided edge guid,
                    None => EdgeState::AddBoth,                  // add both this is normal
                }
            }
        };

        match action {
            EdgeState::Good => return,
            EdgeState::UpdateEdges(_, _) => (),
            EdgeState::AddRight(_) => (),
            EdgeState::AddLeft(_) => (),
            EdgeState::AddBoth => {
                // Create and add new edge
                let new_edge = Rc::new(Edge::new());
                self.add_edge(Rc::clone(&new_edge));
                self.edge_connections.insert(
                    new_edge.get_guid(),
                    (connection.0.get_guid(), connection.1.get_guid()),
                );
                // Associate left hashmap with right node
                let mut l = HashMap::new();
                l.insert(connection.0.get_guid(), new_edge.get_guid());
                self.node_connections.insert(connection.1.get_guid(), l);
                // Associate right hashmap with left node
                let mut r = HashMap::new();
                r.insert(connection.1.get_guid(), new_edge.get_guid());
                self.node_connections.insert(connection.0.get_guid(), r);
            }
        }
    }

    // pub fn remove_connection(&mut self, connection: (Rc<Node>, Rc<Node>)) {}

    // pub fn remove_connection_by_edge(&mut self, connection: Rc<Edge>) {}

    pub fn connect_all_nodes(&mut self) {
        let mut h: Vec<Rc<Node>> = Vec::new();
        for src in self.nodes.values() {
            h.push(Rc::clone(src));
        }
        println!("Original Vector Length: {} items.", h.len());
        for i in { 0..h.len() } {
            for j in { i + 1..h.len() } {
                self.add_connection((Rc::clone(&h[i]), Rc::clone(&h[j])));
            }
        }
        println!("Edges Created: {}.", self.edges.len());
    }
}
