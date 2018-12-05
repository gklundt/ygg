use crate::uuid::guid_64::Guid;
use crate::metrics::uom;

use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Edge {
    guid: Rc<Guid>,
    distance: Option<uom::distance::DistanceKind>,
}

impl Edge {
    pub fn new(distance: Option<uom::distance::DistanceKind>) -> Self {
        Edge { guid: Guid::new(), distance }
    }

    pub fn get_guid(&self) -> Rc<Guid> {
        Rc::clone(&self.guid)
    }

    pub fn get_distance(&self) -> &Option<uom::distance::DistanceKind> {
        &self.distance
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge (guid: {}, distance: {:?})", self.guid.to_string(), self.distance)
    }
}
