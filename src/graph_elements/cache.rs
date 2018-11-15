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
                self.push_edge(nodes.clone());
                let ret = self.contains_cycle();
                self.node_pairs.pop();
                ret
            }
        }
    }

    pub fn contains_cycle(&self) -> bool {
        let mut params = (Vec::new(), Vec::new(), false);
        match self.node_pairs.last() {
            None => false,
            Some(first) => {
                params.0.push(first.get_left());
                params = self.contains_cycle_(first.get_left(), params);
                params.2
            }
        }
    }

    fn contains_cycle_(&self, current_node: Rc<Guid>, mut params: (Vec<Rc<Guid>>, Vec<Rc<NodePair>>, bool)) -> (Vec<Rc<Guid>>, Vec<Rc<NodePair>>, bool) {
        for node_pair in self.node_pairs.clone() { // for each pair in the cache
            if let true = node_pair.contains(current_node.clone()) { // if the pair contains the current node
                if let true = params.1.clone().contains(&node_pair) { continue; } // if the used paths list contains the existing path, skip
                if let Some(peer) = node_pair.get_peer(current_node.clone()) { // get the peer of the current node
                    if let true = params.0.clone().contains(&peer.clone()) { // if the peer has been visited, leave
                        params.2 = true;
                        break;
                    } else {
                        params.0.push(peer.clone()); // push peer onto visited
                        params.1.push(node_pair.clone()); // declare this path used by adding it to the list
                        params = self.contains_cycle_(peer.clone(), params);
                    }
                }
            }
        }
        params
    }
}
