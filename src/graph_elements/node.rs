use crate::uuid::guid_64::Guid;
use crate::graph_elements::location;

use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    name: Option<String>,
    guid: Rc<Guid>,
    location: Option<location::LocationKind>,
}

impl Node {
    pub fn new(location: Option<location::LocationKind>, name: Option<String>) -> Rc<Node> {
        Rc::new(Node {
            name,
            guid: Guid::new(),
            location,
        })
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
