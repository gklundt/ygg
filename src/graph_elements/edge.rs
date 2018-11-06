use crate::uuid::guid_64::Guid;
use crate::metrics::uom;

use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Edge {
    guid: Rc<Guid>,
    distance: Option<uom::DistanceKind>,
}

impl Edge {
    pub fn new(distance: Option<uom::DistanceKind>) -> Self {
        Edge { guid: Guid::new(), distance }
    }

    pub fn get_guid(&self) -> Rc<Guid> {
        Rc::clone(&self.guid)
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge ({})", self.guid.to_string())
    }
}