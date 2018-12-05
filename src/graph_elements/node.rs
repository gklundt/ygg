use crate::uuid::guid_64::Guid;
use crate::metrics::uom::position;

use std::fmt;
use std::rc::Rc;
use std::str;

#[derive(Debug)]
pub struct Node {
    name: Option<String>,
    guid: Rc<Guid>,
    position: Option<position::PositionKind>,
}

impl Node {
    pub fn new(position: Option<position::PositionKind>, name: Option<String>) -> Rc<Node> {
        Rc::new(Node {
            name,
            guid: Guid::new(),
            position,
        })
    }

    pub fn get_guid(&self) -> Rc<Guid> {
        Rc::clone(&self.guid)
    }

    pub fn get_position(&self) -> &position::PositionKind {
        match &self.position {
            Some(pos) => &pos,
            None => &position::PositionKind::Unknown,
        }
    }

    pub fn get_name(&self) -> String{

        match &self.name {
            Some(name) => name.clone(),
            None => (str::to_string("FNU")).clone(),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.name {
            Some(name) => {
                match &self.position {
                    Some(pos) => { write!(f, "( name: {}, guid: {}, position: {} )", name, self.guid.to_string(), pos) }
                    None => { { write!(f, "( name: {}, guid: {}, position: None )", name, self.guid.to_string()) } }
                }
            }
            None => {
                match &self.position {
                    Some(pos) => { write!(f, "( guid: {}, position: {} )", self.guid.to_string(), pos) }
                    None => { write!(f, "( guid: {}, position: None )", self.guid.to_string()) }
                }
            }
        }
    }
}
