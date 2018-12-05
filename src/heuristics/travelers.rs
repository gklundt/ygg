use crate::metrics::uom::UnitOfMeasureValueKind;
use crate::heuristics::resources::Resource;

#[derive(Debug)]
pub struct Traveler<T: UnitOfMeasureValueKind + ?Sized> {
    name: String,
    resources: Vec<Box<Resource<T>>>,
}

pub trait TravelerTrait<T: UnitOfMeasureValueKind + ?Sized> {
    fn push_resource(&mut self, resource: Box<Resource<T>>);
    fn get_resources(&mut self) -> &Vec<Box<Resource<T>>>;
}

impl<T: UnitOfMeasureValueKind + ?Sized> TravelerTrait<T> for Traveler<T> {
    fn push_resource(&mut self, resource: Box<Resource<T>>) {
        self.resources.push(resource);
    }

    fn get_resources(&mut self) -> &Vec<Box<Resource<T>>> {
        &self.resources
    }
}

impl<T: UnitOfMeasureValueKind + ?Sized> Traveler<T> {
    pub fn new(name: String) -> Self {
        Traveler {
            name,
            resources: Vec::new(),
        }
    }
}


