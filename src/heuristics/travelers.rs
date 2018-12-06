use crate::metrics::uom::UnitOfMeasureValueKind;
use crate::heuristics::resources::ResourceTrait;
use std::marker::PhantomData;
use std::collections::HashMap;


#[derive(Debug)]
pub struct Traveler<T: UnitOfMeasureValueKind + ?Sized, R: ResourceTrait<T> + ?Sized> {
    name: String,
    resources: Vec<Box<R>>,
    phantom: PhantomData<T>,
    resources_hash: HashMap<String, Box<R>>,
}

pub trait TravelerTrait<T: UnitOfMeasureValueKind + ?Sized, R: ResourceTrait<T> + ?Sized> {
    fn push_resource(&mut self, resource: Box<R>);
    fn get_resources(&self) -> &Vec<Box<R>>;
}

impl<T: UnitOfMeasureValueKind + ?Sized, R: ResourceTrait<T> + ?Sized> TravelerTrait<T, R> for Traveler<T, R> {
    fn push_resource(&mut self, resource: Box<R>) {
        self.resources.push(resource);
    }

    fn get_resources(&self) -> &Vec<Box<R>> {
        &self.resources
    }
}

impl<T: UnitOfMeasureValueKind + ?Sized, R: ResourceTrait<T> + ?Sized> Traveler<T, R> {
    pub fn new(name: String) -> Self {
        Traveler {
            name,
            resources: Vec::new(),
            phantom: PhantomData,
            resources_hash: HashMap::new(),
        }
    }
}


