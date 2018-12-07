use crate::uuid::guid_64::Guid;
use crate::metrics::uom;

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;


#[derive(Debug)]
pub struct Edge<Cost: uom::UnitOfMeasureValueKind + ?Sized> {
    guid: Rc<Guid>,
    distance: Option<uom::distance::DistanceKind>,
    cost: RefCell<Option<Box<Cost>>>,
}

pub trait CostBasedEdge: Debug{
    type CostType;
    fn set_cost(&self, cost: Self::CostType);
    fn get_cost(&self) -> Option<Self::CostType>;
}

impl<Cost: uom::UnitOfMeasureValueKind> CostBasedEdge for Edge<Cost> {
    type CostType = Cost;

    fn set_cost(&self, cost: <Self as CostBasedEdge>::CostType) {
        let b = Box::new(cost);
        self.cost.replace(Some(b));
    }

    fn get_cost(&self) -> Option<<Self as CostBasedEdge>::CostType> {
        match self.cost.borrow().deref() {
            Some(n) => Some(n.duplicate()),
            None => None
        }
    }
}

impl<Cost: uom::UnitOfMeasureValueKind + ?Sized> Edge<Cost> {
    pub fn new(distance: Option<uom::distance::DistanceKind>) -> Self {
        Edge { guid: Guid::new(), distance, cost: RefCell::new(None) }
    }

    pub fn get_guid(&self) -> Rc<Guid> {
        Rc::clone(&self.guid)
    }

    pub fn get_distance(&self) -> &Option<uom::distance::DistanceKind> {
        &self.distance
    }
}

impl<Cost: uom::UnitOfMeasureValueKind + ?Sized> fmt::Display for Edge<Cost> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Edge (guid: {}, distance: {:?}, cost: {:?})", self.guid.to_string(), self.distance, self.cost)
    }
}
