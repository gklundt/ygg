use crate::uuid::guid_64::Guid;
use std::rc::Rc;
use crate::metrics::uom::UnitOfMeasureValueKind;
use crate::metrics::uom::VolumeKind;
use std::ops::Add;
use std::ops::AddAssign;


#[derive(Debug)]
pub struct Resource<T>
    where T: UnitOfMeasureValueKind
{
    name: String,
    guid: Rc<Guid>,
    capacity: T,
    modifications: Vec<T>,
}

impl<T> Resource<T>
    where T: UnitOfMeasureValueKind
{
    pub fn new(name: String, capacity: T) -> Self {
        Resource { name, guid: Guid::new(), capacity, modifications: Vec::new() }
    }
    pub fn get_capacity(&self) -> &T {
        &self.capacity
    }
    pub fn push_modification(&mut self, modification: T) {
        &self.modifications.push(modification);
    }
//    pub fn get_current_capacity(&self) -> &T {
//        let mut cap: Add<T> = &self.capacity as Add;
//        for modification in self.modifications {
//            cap = cap + modification;
//        }
//        cap
//    }
}

#[cfg(tests)]
mod tests {
    use crate::heuristics::resources::Resource;
    use crate::metrics::uom::DistanceKind;

    #[test]
    fn test_this() {
        let r = Resource::new("Soy yo".to_string(), DistanceKind::Meters(12.1));
    }
}





