use super::super::uuid::Guid;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    guid: Rc<Guid>,
}

impl Node {
    pub fn new() -> Rc<Node> {
        Rc::new(Node { guid: Guid::new() })
    }

    pub fn get_guid(&self) -> Rc<Guid> {
        Rc::clone(&self.guid)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node ({})", self.guid.to_string())
    }
}
