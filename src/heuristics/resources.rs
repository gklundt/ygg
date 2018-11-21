use crate::uuid::guid_64::Guid;
use std::rc::Rc;
use crate::metrics::uom::UnitOfMeasureValueKind;
use crate::metrics::uom::VolumeKind;


#[derive(Debug)]
pub struct Resource<T>
    where T: UnitOfMeasureValueKind
{
    name: String,
    guid: Rc<Guid>,
    capacity: T,
}

impl<T> Resource<T> where T: UnitOfMeasureValueKind {
    pub fn new(name: String, capacity: T) -> Self {
        Resource { name, guid: Guid::new(), capacity }
    }
}





