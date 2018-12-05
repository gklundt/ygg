use crate::metrics::uom::UnitOfMeasureValueKind;
use crate::heuristics::resources::ResourceTrait;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Traveler<T: UnitOfMeasureValueKind + ?Sized, R: ResourceTrait<T> + ?Sized> {
    name: String,
    resources: Vec<Box<R>>,
    phantom: PhantomData<T>
}

pub trait TravelerTrait<T: UnitOfMeasureValueKind + ?Sized, R: ResourceTrait<T> + ?Sized> {
    fn push_resource(&mut self, resource: Box<R>);
    fn get_resources(&mut self) -> &Vec<Box<R>>;
}

impl<T: UnitOfMeasureValueKind + ?Sized, R: ResourceTrait<T> + ?Sized> TravelerTrait<T, R> for Traveler<T, R> {
    fn push_resource(&mut self, resource: Box<R>) {
        self.resources.push(resource);
    }

    fn get_resources(&mut self) -> &Vec<Box<R>> {
        &self.resources
    }
}

impl<T: UnitOfMeasureValueKind + ?Sized, R: ResourceTrait<T> + ?Sized> Traveler<T, R> {
    pub fn new(name: String) -> Self {
        Traveler {
            name,
            resources: Vec::new(),
            phantom: PhantomData,
        }
    }
}


