use crate::solutions;
use crate::metrics::uom::DistanceKind;
use crate::metrics::uom::Si;
use std::cmp::Ordering;
use std::rc::Rc;
use crate::graph_elements::cache::TreeCache;
use crate::graph_elements::node_pair::NodePair;

pub fn solve(solution: &mut solutions::Solution, problem: &solutions::ProblemKind) {
    if let solutions::ProblemKind::MinimumSpanningTree { graph_guid: g } = problem {
        if let Some(og) = solution.get_graph().get_graph(g.clone()) {
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


            let node_count = my_graph.get_nodes().len();
            let mut tree_cache = TreeCache::new();
            for edge_distance in edge_distances {
                let node_pair = Rc::new(NodePair::new((edge_distance.0, edge_distance.2)));
                if let false = tree_cache.check_for_cycle(node_pair.clone()) {
                    println!("Adding {} -> {}", node_pair.get_left().clone(), node_pair.get_right().clone());
                    my_graph.add_connected_nodes_by_guid(node_pair.clone());
                    tree_cache.push_edge(node_pair.clone());
                } else {
                    println!("Declining {} -> {}", node_pair.get_left().clone(), node_pair.get_right().clone());
                }
                if let true = tree_cache.node_pairs().len() - 1 == node_count { break; }
            }

            solution.add_sub_graph(Rc::new(my_graph));
        } // find the right sub-graph from the solution
    } // make sure we're solving the right problem
}

