use crate::solutions;
use crate::metrics::uom::DistanceKind;
use crate::metrics::uom::Si;
use std::cmp::Ordering;
use std::rc::Rc;


pub fn solve(solution: &mut solutions::Solution, problem: &solutions::ProblemKind) {
    if let solutions::ProblemKind::GreedyTour { graph_guid: g } = problem {
        if let Some(og) = solution.get_graph().get_graph(g.clone()) {
            let mut edge_distances = Vec::new();

            let mut my_graph = og.replicate_all();
            let re_hash_edges = my_graph.get_edges().clone();
            let re_hash_connections = my_graph.get_edge_connections().clone();

            for edge in re_hash_edges.iter() {
                if let Some(edge_connection) = re_hash_connections.get(edge.0) {
                    if let Some(distance_kind) = edge.1.get_distance() {
                        if let DistanceKind::Meters(distance) = distance_kind.to_si() {
                            let val = (edge_connection.0.clone(), edge.0.clone(), edge_connection.1.clone(), distance);
                            edge_distances.push(val);
                        } else { return; } // SI unit is not meters for some reason.
                    } else { return; } // Distance must be set for edge or this doesn't work
                } else { return; } // Edge connection does not exist for this edge reference.
            }

            edge_distances.sort_by(|a, b|
                match a.3.partial_cmp(&b.3) {
                    Some(ordering) => { ordering }
                    None => { Ordering::Equal }
                });

            my_graph.remove_all_edges();

            for edge_distance in edge_distances {
                let i = my_graph.get_degree(edge_distance.0.clone()) as u32;
                let j = my_graph.get_degree(edge_distance.2.clone()) as u32;

                let zero_test = i == 0 || j == 0;
                let two_test = i < 2 && j < 2;
                let add_test = zero_test && two_test;
                if let true = add_test {
                    my_graph.add_connected_node_guids((edge_distance.0.clone(), edge_distance.2.clone()));
                }

                let path_test = i == 1 && j == 1;
                if let true = path_test {
                    if let Some(p) = my_graph.get_path_for_node(edge_distance.0.clone()) {
                        if let false = p.contains(&edge_distance.2.clone()) {
                            my_graph.add_connected_node_guids((edge_distance.0.clone(), edge_distance.2.clone()));
                        }
                    }
                }
            }
            solution.add_sub_graph(Rc::new(my_graph));
        } // find the right sub-graph from the solution
    } // make sure we're solving the right problem
}


