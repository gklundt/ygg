use crate::uuid::guid_64::Guid;
use crate::metrics::uom::UnitOfMeasureValueKind;
use std::rc::Rc;
use core::borrow::Borrow;
use std::fmt::Debug;


#[derive(Debug, Clone)]
pub struct Resource<T: UnitOfMeasureValueKind + ?Sized> {
    name: String,
    guid: Rc<Guid>,
    min: Box<T>,
    max: Box<T>,
    modifications: Vec<Box<T>>,
    starting_value: Box<T>,
    current_value: Box<T>,
}

pub trait ResourceTrait<T: UnitOfMeasureValueKind + ?Sized>: Debug {}

impl<T: UnitOfMeasureValueKind> ResourceTrait<dyn UnitOfMeasureValueKind> for Resource<T> {}

impl<T: UnitOfMeasureValueKind + ?Sized> ResourceTrait<dyn UnitOfMeasureValueKind> for T {}

impl<T: UnitOfMeasureValueKind> Resource<T> {
    pub fn new(name: String, min: Box<T>, max: Box<T>, starting_value: Box<T>) -> Self where T: Sized {
        let min_si = Box::new(min.duplicate().as_standard_unit().duplicate());
        let max_si = Box::new(max.duplicate().as_standard_unit().duplicate());
        let sv_si = Box::new(starting_value.duplicate().as_standard_unit().duplicate());
        let cv_si = Box::new(starting_value.duplicate().as_standard_unit().duplicate());

        Resource {
            name,
            guid: Guid::new(),
            min: min_si,
            max: max_si,
            modifications: Vec::new(),
            starting_value: sv_si,
            current_value: cv_si,
        }
    }

    pub fn get_min(&self) -> &T {
        self.min.borrow()
    }

    pub fn get_max(&self) -> &T {
        self.max.borrow()
    }


    pub fn push_modification(&mut self, modification: Box<T>) {
        &self.modifications.push(Box::new(modification.duplicate().as_standard_unit().duplicate()));
    }


    pub fn get_current_value(&mut self) -> &T {
        let mut ledger_value = 0.0;
        for modification in &self.modifications {
            ledger_value += modification.get_value().unwrap();
        }

        let original_value = self.starting_value.get_value().unwrap();
        let current_value = original_value + ledger_value;
        self.current_value.set_value(current_value);
        self.current_value.borrow()
    }
}











