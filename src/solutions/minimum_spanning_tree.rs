use crate::solutions;
use crate::metrics::uom::DistanceKind;
use crate::metrics::uom::Si;
use std::cmp::Ordering;
use std::rc::Rc;
use crate::graph_elements::cache::TreeCache;
use crate::graph_elements::node_pair::NodePair;

pub fn solve(solution: &mut solutions::Solution, problem: &solutions::ProblemKind) {
    if let solutions::ProblemKind::MinimumSpanningTree { graph_guid: graph_guid } = problem {
        if let Some(og) = solution.get_graph().get_graph(graph_guid.clone()) {
            let mut edge_distances = Vec::new();


            let mut my_graph = og.replicate_all();
            let re_hash_edges = my_graph.get_edges().clone();
            let re_hash_connections = my_graph.get_edge_connections().clone();

            for edge in re_hash_edges.iter() {
                if let Some(edge_connection) = re_hash_connections.get(edge.0) {
                    if let Some(distance_kind) = edge.1.get_distance() {
                        if let DistanceKind::Meters(distance) = distance_kind.to_si() {
                            let val = (edge_connection.get_pair().0.clone(), edge.0.clone(), edge_connection.get_pair().1.clone(), distance);
                            edge_distances.push(val);
                        } else { return; } // SI unit is not meters for some reason.
                    } else { return; } // Distance must be set for edge or this doesn't work
                } else { return; } // Edge connection does not exist for this edge reference.
            }

            edge_distances.sort_by(|a, b|
                {
                    match a.3.partial_cmp(&b.3) {
                        Some(ordering) => { ordering }
                        None => { Ordering::Less }
                    }
                });

            my_graph.remove_all_edges();


            let node_count = my_graph.get_nodes().len() as i32;
            let mut edge_count = -1;
            let mut tree_cache = TreeCache::new();
            for edge_distance in edge_distances {
                if let true = edge_count == node_count { break; }

                let node_pair = Rc::new(NodePair::new((edge_distance.0, edge_distance.2)));

                let i = my_graph.get_degree_of_node(node_pair.get_left()) as u32;
                let j = my_graph.get_degree_of_node(node_pair.get_right()) as u32;

                let zero_test = i == 0 || j == 0;
                let two_test = i < 2 && j < 2;
                let add_test = zero_test && two_test;
                if let true = add_test {
                    my_graph.add_connected_nodes_by_guid(node_pair.clone());
                    tree_cache.push_edge(node_pair.clone());
                    edge_count += 1;
                    continue;
                }


                if let false = tree_cache.check_for_cycle(node_pair.clone()) {
                    my_graph.add_connected_nodes_by_guid(node_pair.clone());
                    tree_cache.push_edge(node_pair.clone());
                    edge_count += 1;
                }
            }
            solution.add_sub_graph(Rc::new(my_graph));
        } // find the right sub-graph from the solution
    } // make sure we're solving the right problem
}

