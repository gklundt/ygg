use crate::uuid::guid_64::Guid;
use crate::metrics::uom;

use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    name: Option<String>,
    guid: Rc<Guid>,
    position: Option<uom::PositionKind>,
}

impl Node {
    pub fn new(position: Option<uom::PositionKind>, name: Option<String>) -> Rc<Node> {
        Rc::new(Node {
            name,
            guid: Guid::new(),
            position,
        })
    }

    pub fn get_guid(&self) -> Rc<Guid> {
        Rc::clone(&self.guid)
    }

    pub fn get_position(&self) -> &uom::PositionKind {
        match &self.position {
            Some(pos) => &pos,
            None => &uom::PositionKind::Unknown,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node ( name: {:?}, guid: {}, position: {:?} )", self.name, self.guid.to_string(), self.position)
    }
}
