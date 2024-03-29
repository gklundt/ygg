mod shortest_path;
mod greedy_tour;
mod build_routes;
mod optimize_within_routes;
mod optimize_between_routes;
mod minimum_spanning_tree;
mod solvers;

use std::rc::Rc;
use std::fmt;
use crate::uuid::guid_64::Guid;
use crate::graph_elements::graph::Graph;
use crate::metrics::uom::UnitOfMeasureValueKind;


#[derive(Debug)]
pub enum ProblemKind {
    ShortestPath { graph_guid: Rc<Guid>, start_node_guid: Rc<Guid>, end_node_guid: Rc<Guid> },
    GreedyTour { graph_guid: Rc<Guid> },
    MinimumSpanningTree { graph_guid: Rc<Guid> },
    BuildRoutes { graph_guid: Rc<Guid> },
    OptimizeWithinRoutes { graph_guid: Rc<Guid> },
    OptimizeBetweenRoutes { graph_guid: Rc<Guid> },
    Undefined,
}

impl fmt::Display for ProblemKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ProblemKind::ShortestPath { graph_guid: _, start_node_guid: _, end_node_guid: _ } => write!(f, "( ProblemKind::ShortestPath )"),
            ProblemKind::GreedyTour { graph_guid: _ } => write!(f, "( ProblemKind::ShortestPath )"),
            ProblemKind::BuildRoutes { graph_guid: _ } => write!(f, "( ProblemKind::BuildRoutes )"),
            ProblemKind::OptimizeWithinRoutes { graph_guid: _ } => write!(f, "( ProblemKind::OptimizeWithinRoutes )"),
            ProblemKind::OptimizeBetweenRoutes { graph_guid: _ } => write!(f, "( ProblemKind::OptimizeBetweenRoutes )"),
            ProblemKind::Undefined {} => write!(f, "( ProblemKind::Undefined )"),
            ProblemKind::MinimumSpanningTree { graph_guid: _ } => write!(f, "( ProblemKind::MinimumSpanningTree )"),
            //_ => write!(f, "( No Display )"),
        }
    }
}


#[derive(Debug)]
pub struct Solution<Cost: UnitOfMeasureValueKind + ?Sized> {
    graph: Graph<Cost>,
}


impl<Cost: UnitOfMeasureValueKind + ?Sized> Solution<Cost> {
    pub fn new(graph: Graph<Cost>) -> Self {
        Solution { graph }
    }

    pub fn solve(&mut self, problem: &ProblemKind) {
        match problem {
            ProblemKind::ShortestPath { graph_guid: _, start_node_guid: _, end_node_guid: _ } => self.solve_(problem, shortest_path::solve),
            ProblemKind::GreedyTour { graph_guid: _ } => self.solve_(problem, greedy_tour::solve),
            ProblemKind::BuildRoutes { graph_guid: _ } => self.solve_(problem, build_routes::solve),
            ProblemKind::OptimizeWithinRoutes { graph_guid: _ } => self.solve_(problem, optimize_within_routes::solve),
            ProblemKind::OptimizeBetweenRoutes { graph_guid: _ } => self.solve_(problem, optimize_between_routes::solve),
            ProblemKind::MinimumSpanningTree { graph_guid: _ } => self.solve_(problem, minimum_spanning_tree::solve),
            _ => (),
        }
    }

    fn solve_(&mut self, problem: &ProblemKind, field: fn(&mut Solution<Cost>, &ProblemKind)) {
        field(self, problem);
    }

    pub fn get_graph(&self) -> &Graph<Cost> {
        &self.graph
    }

    pub fn add_sub_graph(&mut self, sub_graph: Rc<Graph<Cost>>) {
        self.graph.add_sub_graph(sub_graph);
    }
}
