use crate::uuid::guid_64::Guid;
use std::rc::Rc;
use crate::graph_elements::node_pair::NodePair;


pub struct TreeCache {
    node_pairs: Vec<Rc<NodePair>>,
    paths: Vec<(Vec<Rc<Guid>>, Vec<Rc<NodePair>>)>,
//    metrics: CacheMetrics,

}

//struct CacheMetrics {
//    hits: u32,
//    misses: u32,
//}

impl TreeCache {
    pub fn new() -> Self {
        TreeCache {
            node_pairs: Vec::new(),
            paths: Vec::new(),
//            metrics: CacheMetrics { hits: 0, misses: 0 },
        }
    }

    pub fn node_pairs(&self) -> &Vec<Rc<NodePair>> {
        &self.node_pairs // all known edges, consequently all visited paths
    }

    pub fn push_edge(&mut self, nodes: Rc<NodePair>) {
        if let false = &self.node_pairs.contains(&nodes) {
            &self.node_pairs.push(nodes);
        }
    }

    pub fn check_for_cycle(&mut self, nodes: Rc<NodePair>) -> bool {
        /*First question I should ask is :
        Are there any paths that contain this potential edge?
        Paths is a collection of subsets of the known paths ...*/

        let mut ret = false; // assume no cycle
        for path in &self.paths {
            if let true = path.1.contains(&nodes.clone()) { ret = true }
        }

        if let true = ret { ret } else {
            /* There are no known paths for this node pair (edge), so start the traversal
            ** If a tree edge is found, it will be pushed through the public method.*/
            self.push_edge(nodes.clone());
            let ret = self.contains_cycle();
            self.node_pairs.pop();
            ret
        }
    }

    pub fn contains_cycle(&mut self) -> bool {
        let mut params = (Vec::new(), Vec::new(), false);
        match self.node_pairs.last() {
            None => false,
            Some(first) => {
                let mut candidate_node = first.get_left();

                // Use best existing path if one exists
                // find all paths that have the nodes I care about
                let mut candidate_paths = Vec::new();
                let mut index: usize = 0;
                for path in &self.paths {
                    if let true = path.0.contains(&first.get_left()) { candidate_paths.push((index, path.0.len(), first.get_left())) }
                    if let true = path.0.contains(&first.get_right()) { candidate_paths.push((index, path.0.len(), first.get_right())) }
                    index += 1;
                }

                // longest one should be the most relevant
                candidate_paths.sort_by(|a, b| a.1.cmp(&b.1));
                if let Some(t) = candidate_paths.last() {
                    unsafe {
                        let data: &(Vec<Rc<Guid>>, Vec<Rc<NodePair>>) = self.paths.get_unchecked(t.0);
                        params.0 = data.0.to_vec();
                        params.1 = data.1.to_vec();
                        candidate_node = t.clone().2;
                    }
                } else {
                    params.0.push(candidate_node.clone());
                }

                params = self.contains_cycle_(candidate_node.clone(), params);
                // Preserve the path if it is proven non cyclic ...
                if let false = params.2 {
                    self.paths.push((params.0, params.1));
                }
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
