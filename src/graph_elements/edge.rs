use super::super::uuid::Guid;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Edge {
    guid: Rc<Guid>,
}

impl Edge {
    pub fn new() -> Rc<Edge> {
        Rc::new(Edge { guid: Guid::new() })
    }

    pub fn get_guid(&self) -> Rc<Guid>{
        Rc::clone(&self.guid)
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge ({})", self.guid.to_string())
    }
}
