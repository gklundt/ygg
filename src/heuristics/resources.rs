use crate::uuid::guid_64::Guid;
use crate::metrics::uom::UnitOfMeasureValueKind;
use std::rc::Rc;
use core::borrow::Borrow;

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

impl<T: UnitOfMeasureValueKind> Resource<T> {
    pub fn new(name: String, min: Box<T>, max: Box<T>, starting_value: Box<T>) -> Self {
        let min_si = Box::new(min.clone().as_standard_unit().clone());
        let max_si = Box::new(max.clone().as_standard_unit().clone());
        let sv_si = Box::new(starting_value.clone().as_standard_unit().clone());
        let cv_si = Box::new(starting_value.clone().as_standard_unit().clone());

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
        &self.modifications.push(Box::new(modification.clone().as_standard_unit().clone()));
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






