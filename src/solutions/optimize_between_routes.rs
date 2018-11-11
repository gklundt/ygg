use crate::uuid::guid_64::Guid;
use crate::solutions;
use std::rc::Rc;

pub fn solve(solution: &mut solutions::Solution, problem: &solutions::ProblemKind) {
    let graph: Rc<Guid>;
    if let solutions::ProblemKind::OptimizeBetweenRoutes { graph_guid: g } = problem {
        graph = g.clone();
        println!("Problem: {:?} \n{:?}\n{:?}, ", problem, solution, graph);
    }

}
