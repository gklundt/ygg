use crate::uuid::guid_64::Guid;
use std::rc::Rc;
use crate::graph_elements::node_pair::NodePair;


pub struct TreeCache {
    node_pairs: Vec<Rc<NodePair>>,
}

impl TreeCache {
    pub fn new() -> Self {
        TreeCache { node_pairs: Vec::new() }
    }

    pub fn node_pairs(&self) -> &Vec<Rc<NodePair>> {
        &self.node_pairs
    }

    pub fn push_edge(&mut self, nodes: Rc<NodePair>) {
        if let false = &self.node_pairs.contains(&nodes) {
            &self.node_pairs.push(nodes);
        }
    }

    pub fn check_for_cycle(&mut self, nodes: Rc<NodePair>) -> bool {
        match self.node_pairs.contains(&nodes) {
            true => true,
            false => {
                &self.push_edge(nodes);
                let ret = self.contains_cycle();
                &self.node_pairs.pop();
                ret
            }
        }
    }

    pub fn contains_cycle(&self) -> bool {
        let mut params = (Vec::new(), Vec::new(), false);
        match &self.node_pairs.first() {
            None => false,
            Some(first) => {
                params = self.contains_cycle_(first.get_pair().0.clone(), params);
                params.2
            }
        }
    }

    fn contains_cycle_(&self, current_node: Rc<Guid>, mut params: (Vec<Rc<Guid>>, Vec<Rc<NodePair>>, bool)) -> (Vec<Rc<Guid>>, Vec<Rc<NodePair>>, bool) {

        params.0.push(current_node.clone());
        for node_pair in &self.node_pairs {
            if let true = params.1.clone().contains(node_pair) { continue; }
            if let true = node_pair.contains(current_node.clone()) {
                if let Some(peer) = node_pair.get_peer(current_node.clone()) {
                    params.1.push(node_pair.clone());
                    if let true = params.0.clone().contains(&peer.clone()) { params.2 = true; } else {
                        params =self.contains_cycle_(peer.clone(), params);
                    }
                }
            }
            if let true = params.2 { break; }
        }
        params
    }
}
