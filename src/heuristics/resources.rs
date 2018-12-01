use crate::uuid::guid_64::Guid;
use crate::metrics::uom::UnitOfMeasureValueKind;
use std::rc::Rc;
//use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Resource<T: UnitOfMeasureValueKind> {
    name: String,
    guid: Rc<Guid>,
    min: Box<T>,
    max: Box<T>,
    modifications: Vec<Box<T>>,
    starting_value: Box<T>,

}


impl<T> Resource<T>
    where T: UnitOfMeasureValueKind, T: std::fmt::Debug {
    pub fn new(name: String, mut min: T, mut max: T, mut starting_value: T) -> Self {
        min.as_standard_unit();
        max.as_standard_unit();
        starting_value.as_standard_unit();

        let m = Box::new(min.clone());
        let n = Box::new(max.clone());
        let o = Box::new(starting_value.clone());
        Resource {
            name,
            guid: Guid::new(),
            min: m,
            max: n,
            modifications: Vec::new(),
            starting_value: o,
        }
    }

    pub fn get_min(&self) -> T {
        *self.min.clone()
    }

    pub fn get_max(&self) -> T {
        *self.max.clone()
    }

    pub fn push_modification(&mut self, mut modification: T) {
        &self.modifications.push(Box::new(modification.as_standard_unit().clone()));
    }

    pub fn get_current_value(&mut self) -> T {
        let mut ledger: Vec<f64> = Vec::new();
        for modification in &self.modifications {
            let i = modification.clone().get_value().unwrap();
            ledger.push(i);
        }

        let ledger_value: f64 = ledger.iter().sum();
        let original_value = self.starting_value.get_value().unwrap();
        let current_value = original_value + ledger_value;
        self.starting_value.clone().set_value(current_value).clone()
    }
}

#[cfg(tests)]
mod tests {
    use crate::heuristics::resources::Resource;
    use crate::metrics::uom::DistanceKind;
    use crate::metrics::uom::distance::DistanceKind;

    #[test]
    fn test_this() {
        let r = Resource::new("Soy yo".to_string(),
                              DistanceKind::Meters(0.0),
                              DistanceKind::Meters(100.0),
                              DistanceKind::Meters(0.0));
    }
}





