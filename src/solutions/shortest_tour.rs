use crate::solutions;

use crate::metrics::uom::DistanceKind;
use crate::metrics::uom::Si;
use crate::graph_elements::edge::Edge;

pub fn solve(solution: &mut solutions::Solution, problem: &solutions::ProblemKind) {
    if let solutions::ProblemKind::ShortestTour { graph: g } = problem {


        // create working graph from the solution graph
        let og = solution.get_graph();
        let mut my_graph = og.replicate();

        //println!("{} ?= {}", og.get_guid(), my_graph.get_guid());
    }
}


