use crate::uuid::guid_64::Guid;
use std::rc::Rc;

#[derive(Debug)]
pub struct NodePair { left: Rc<Guid>, right: Rc<Guid> }

impl PartialEq for NodePair {
    fn eq(&self, other: &NodePair) -> bool {
        match (Rc::ptr_eq(&self.left, &other.get_left()),
               Rc::ptr_eq(&self.right, &other.get_right()),
               Rc::ptr_eq(&self.left, &other.get_right()),
               Rc::ptr_eq(&self.right, &other.get_left())
        ) {
            (true, true, _, _) => true,
            (_, _, true, true) => true,
            (_, _, _, _) => false,
        }
    }
}

impl NodePair {
    pub fn new(nodes: (Rc<Guid>, Rc<Guid>)) -> Self {
        NodePair {
            left: nodes.0.clone(),
            right: nodes.1.clone(),
        }
    }

    pub fn get_left(&self) -> Rc<Guid> {
        self.left.clone()
    }

    pub fn get_right(&self) -> Rc<Guid> {
        self.right.clone()
    }

    pub fn get_pair(&self) -> (Rc<Guid>, Rc<Guid>) {
        (self.left.clone(), self.right.clone())
    }

    pub fn contains(&self, node_guid: Rc<Guid>) -> bool {
        Rc::ptr_eq(&self.left, &node_guid) ||
            Rc::ptr_eq(&self.right, &node_guid)
    }

    pub fn get_peer(&self, node_guid: Rc<Guid>) -> Option<Rc<Guid>> {
        match Rc::ptr_eq(&self.left, &node_guid) {
            true => Some(self.right.clone()),
            false => match Rc::ptr_eq(&self.right, &node_guid) {
                true => Some(self.left.clone()),
                false => None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph_elements::node::Node;
    use crate::graph_elements::node_pair::NodePair;

    #[test]
    fn node_pair_contains() {
        let left = Node::new(None, None).get_guid();
        let right = Node::new(None, None).get_guid();
        let other = Node::new(None, None).get_guid();
        let n: NodePair = NodePair {
            left: left.clone(),
            right: right.clone(),
        };
        assert_eq!(true, n.contains(left.clone()));
        assert_eq!(true, n.contains(right.clone()));
        assert_eq!(false, n.contains(other.clone()));
    }
}
