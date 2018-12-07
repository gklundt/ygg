use crate::uuid::guid_64::Guid;
use crate::solutions;
use std::rc::Rc;
use crate::metrics::uom::UnitOfMeasureValueKind;

pub fn solve<Cost: UnitOfMeasureValueKind + ?Sized>(solution: &mut solutions::Solution<Cost>, problem: &solutions::ProblemKind) {
    let graph: Rc<Guid>;
    if let solutions::ProblemKind::OptimizeWithinRoutes { graph_guid: g } = problem {
        graph = g.clone();
        println!("Problem: {:?} \n{:?}\n{:?}, ", problem, solution, graph);
    }

}
