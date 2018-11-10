mod shortest_path;
mod shortest_tour;
mod build_routes;
mod optimize_within_routes;
mod optimize_between_routes;
mod solvers;

use std::rc::Rc;
use std::fmt;
use crate::uuid::guid_64::Guid;
use crate::graph_elements::graph::Graph;


#[derive(Debug)]
pub enum ProblemKind {
    ShortestPath { graph: Rc<Guid>, start_node: Rc<Guid>, end_node: Rc<Guid> },
    ShortestTour { graph: Rc<Guid> },
    BuildRoutes { graph: Rc<Guid> },
    OptimizeWithinRoutes { graph: Rc<Guid> },
    OptimizeBetweenRoutes { graph: Rc<Guid> },
    Undefined,
}

impl fmt::Display for ProblemKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ProblemKind::ShortestPath { graph: _, start_node: _, end_node: _ } => write!(f, "( ProblemKind::ShortestPath )"),
            ProblemKind::ShortestTour { graph: _ } => write!(f, "( ProblemKind::ShortestPath )"),
            ProblemKind::BuildRoutes { graph: _ } => write!(f, "( ProblemKind::BuildRoutes )"),
            ProblemKind::OptimizeWithinRoutes { graph: _ } => write!(f, "( ProblemKind::OptimizeWithinRoutes )"),
            ProblemKind::OptimizeBetweenRoutes { graph: _ } => write!(f, "( ProblemKind::OptimizeBetweenRoutes )"),
            ProblemKind::Undefined {} => write!(f, "( ProblemKind::Undefined )"),
            //_ => write!(f, "( No Display )"),
        }
    }
}

#[derive(Debug)]
pub struct Solution {
    graph: Rc<Graph>,
}

impl Solution {
    pub fn new(graph: Rc<Graph>) -> Self {
        Solution { graph }
    }

    pub fn solve(&mut self, problem: &ProblemKind) {
        match problem {
            ProblemKind::ShortestPath { graph: _, start_node: _, end_node: _ } => self.solve_(problem, shortest_path::solve),
            ProblemKind::ShortestTour { graph: _ } => self.solve_(problem, shortest_tour::solve),
            ProblemKind::BuildRoutes { graph: _ } => self.solve_(problem, build_routes::solve),
            ProblemKind::OptimizeWithinRoutes { graph: _ } => self.solve_(problem, optimize_within_routes::solve),
            ProblemKind::OptimizeBetweenRoutes { graph: _ } => self.solve_(problem, optimize_between_routes::solve),
            _ => (),
        }
    }

    fn solve_(&mut self, problem: &ProblemKind, f: fn(&mut Solution, &ProblemKind)) {
        f(self, problem);
    }

    pub fn get_graph(&self) -> Rc<Graph> {
        self.graph.clone()
    }
}
