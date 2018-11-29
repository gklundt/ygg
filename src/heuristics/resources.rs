use crate::uuid::guid_64::Guid;
use crate::metrics::uom::UnitOfMeasureValueKind;
use crate::metrics::uom::volume::VolumeKind;
use std::rc::Rc;
use std::ops::Add;
use std::ops::AddAssign;

#[derive(Debug)]
pub struct Resource<T>
    where T: UnitOfMeasureValueKind
{
    name: String,
    guid: Rc<Guid>,
    min: T,
    max: T,
    modifications: Vec<T>,
    starting_value: T,

}

impl<T> Resource<T>
    where T: UnitOfMeasureValueKind, T: std::fmt::Debug
{
    pub fn new(name: String, mut min: T, mut max: T, mut starting_value: T) -> Self {
        Resource {
            name,
            guid: Guid::new(),
            min: min.to_si(),
            max: max.to_si(),
            modifications: Vec::new(),
            starting_value: starting_value.to_si(),
        }
    }

    pub fn get_min(&self) -> T {
        let cp = unsafe { core::mem::transmute_copy(&self.min) };
        cp
    }

    pub fn get_max(&self) -> T {
        let cp = unsafe { core::mem::transmute_copy(&self.max) };
        cp
    }

    pub fn get_min_ref(&self) -> &T {
        &self.min
    }

    pub fn get_max_ref(&self) -> &T {
        &self.max
    }

    pub fn push_modification(&mut self, modification: T) {
        &self.modifications.push(modification.to_si());
    }

    pub fn get_current_value(&mut self) -> T {
        let mut ledger: Vec<f64> = Vec::new();

        for modification in &self.modifications {
            let i = modification.get_value().unwrap();
            ledger.push(i);
        }

        let ledger_value: f64 = ledger.iter().sum();
        let original_value = self.starting_value.get_value().unwrap();
        let current_value = original_value + ledger_value;

        self.starting_value.set_value(current_value);
        let cp = unsafe { core::mem::transmute_copy(&self.starting_value) };
        self.starting_value.set_value(original_value);
        cp
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





