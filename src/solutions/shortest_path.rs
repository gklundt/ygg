use crate::uuid::guid_64::Guid;
use crate::solutions;

use std::rc::Rc;

pub fn solve(solution: &mut solutions::Solution, problem: &solutions::ProblemKind) {
    let graph: Rc<Guid>;
    let start: Rc<Guid>;
    let end: Rc<Guid>;
    if let solutions::ProblemKind::ShortestPath { graph: g, start_node: s, end_node: e } = problem {
        graph = g.clone();
        start = s.clone();
        end = e.clone();
        println!("Problem: {},\nSolution: {:?},\nGraph: {},\nStart: {},\nEnd: {}, ", problem, solution, graph, start, end);
    }


    // create working graph
    let og = solution.get_graph();
    let mut my_graph = og.replicate();
    let mut v = Vec::new();

    let mut i = 0;
    for n in my_graph.get_edges() {
        if let false = i % 2 == 0 { v.push(n.1.clone()); }
        else { v.push(n.1.clone());}
        i += 1;
    }
    for n in v {
        my_graph.remove_edge_connection(n);
    }

    println!("{} ?= {}\n{}", og.get_guid(), my_graph.get_guid(), my_graph)
}


