extern crate ygg;

pub fn main() {
    heuristics_sandbox::main();
}

mod heuristics_sandbox {
    use ygg::heuristics::constraints::Constraint;
    use ygg::heuristics::constraints::ConstraintOperations;
    use ygg::heuristics::constraints::ComparisonOperatorKind;
    use ygg::heuristics::constraints::BoolOperatorKind;

    pub fn main() {

        let early_time = 1.0;
        let late_time = 10.0;
        let mut c = Constraint::new("must_be_on_time".to_string());

        let calculated_time = 5.0;

        c.push_const("early_time".to_string(), early_time);
        c.push_var("calculated_time".to_string(), calculated_time);
        c.push_compare_op(ComparisonOperatorKind::Lt);
        c.push_const("late_time".to_string(), late_time);
        c.push_var("calculated_time".to_string(), calculated_time);
        c.push_compare_op(ComparisonOperatorKind::Gt);
        c.push_bool_op(BoolOperatorKind::And);
        let is_on_time = c.evaluate();
        println!("On time: {:?}", is_on_time);

        let calculated_time = 13.0;
        c.mut_var("calculated_time".to_string(), calculated_time);
        let is_on_time = c.evaluate();
        println!("On time: {:?}", is_on_time);

        let calculated_time = 1.01;
        c.mut_var("calculated_time".to_string(), calculated_time);
        let is_on_time = c.evaluate();
        println!("On time: {:?}", is_on_time);

    }

    #[cfg(test)]
    mod tests{
        #[test]
        pub fn test_me(){
            assert_eq!(1,1)
        }
    }
}
/*
mod goofing {
    extern crate ygg;

    //use rand::{thread_rng, Rng};
    use ygg::graph_elements::graph::Graph;
    use ygg::graph_elements::node::Node;
    use ygg::metrics::uom::DistanceKind;
    use ygg::metrics::uom::PositionKind;
    use ygg::solutions::ProblemKind;
    use ygg::solutions::Solution;
    use rand::thread_rng;
    use rand::Rng;
    use ygg::metrics::uom::Si;


    pub fn main_fruff() {
        let mut primer = Graph::new();


//
//
//    let n1: Rc<Node> = Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(80.0), y: DistanceKind::Meters(90.0) }), Some("A".to_string()));
//    let n2: Rc<Node> = Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(60.0), y: DistanceKind::Meters(55.0) }), Some("B".to_string()));
//    let n3: Rc<Node> = Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(45.0), y: DistanceKind::Meters(18.0) }), Some("C".to_string()));
//    let n4: Rc<Node> = Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(38.0), y: DistanceKind::Meters(30.0) }), Some("D".to_string()));
//
//    g.add_node(n1.clone());
//    g.add_node(n2.clone());
//    g.add_node(n3.clone());
//    g.add_node(n4.clone());
//
//    g.add_connected_nodes((n1.clone(), n2.clone()));
//    g.add_connected_nodes((n3.clone(), n2.clone()));
//    g.add_connected_nodes((n4.clone(), n3.clone()));
//    g.add_connected_nodes((n4.clone(), n4.clone()));
//
//    println!("Graph: \n{}", g);
//
//    println!("Tree: \n");
//    if let Some(thing) = g.get_path_for_node(n2.get_guid().clone()) {
//        for t in thing {
//            println!("\tTree Member: {}", t)
//        }
//    } else { println!("Not a Path"); }


        for i in 0..5 {
            let name = format!("Node: {}", i);

            primer.add_node(
                Node::new(
                    Some(
                        PositionKind::TwoDimensionEuclidean {
                            x: DistanceKind::Meters(thread_rng().gen_range(1.0, 100.0) as f64),
                            y: DistanceKind::Meters(thread_rng().gen_range(1.0, 100.0) as f64),
                            //y: DistanceKind::Meters(0.0),
                        }),
                    Some(name),
                )
            );
        }

//
//        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(85.21773885774718), y: DistanceKind::Meters(62.173474906841335) }), Some("Node: 0".to_string())));
//        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(85.3180434700734), y: DistanceKind::Meters(46.95934591522585) }), Some("Node: 1".to_string())));
//        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(19.26007772031716), y: DistanceKind::Meters(98.86595581386) }), Some("Node: 2".to_string())));
//        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(15.357552133032012), y: DistanceKind::Meters(10.593154195440874) }), Some("Node: 3".to_string())));
//        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(50.97348747875345), y: DistanceKind::Meters(32.653332993210746) }), Some("Node: 4".to_string())));
//        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(61.107372713944216), y: DistanceKind::Meters(7.524174645844415) }), Some("Node: 5".to_string())));
//        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(49.923418279681634), y: DistanceKind::Meters(91.09706074700215) }), Some("Node: 6".to_string())));
//        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(1.3503651113306159), y: DistanceKind::Meters(49.45295079739459) }), Some("Node: 7".to_string())));
//        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(2.61376646950271), y: DistanceKind::Meters(48.17944662297873) }), Some("Node: 8".to_string())));
//        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(17.924954051784308), y: DistanceKind::Meters(39.42091381167005) }), Some("Node: 9".to_string())));


        primer.connect_all_nodes();
        //println!("{}", primer);
        println!("{}", primer.get_edge_distance());

//    let n1: Rc<Node> = Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(80.0), y: DistanceKind::Meters(90.0) }), Some("A".to_string()));
//    let n2: Rc<Node> = Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(60.0), y: DistanceKind::Meters(55.0) }), Some("B".to_string()));
//    let n3: Rc<Node> = Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(45.0), y: DistanceKind::Meters(18.0) }), Some("C".to_string()));
//    let n4: Rc<Node> = Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(38.0), y: DistanceKind::Meters(30.0) }), Some("D".to_string()));
//    let n5: Rc<Node> = Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(18.0), y: DistanceKind::Meters(32.0) }), Some("E".to_string()));
//    let n6: Rc<Node> = Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(1.0), y: DistanceKind::Meters(60.0) }), Some("F".to_string()));
//    let n7: Rc<Node> = Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(20.0), y: DistanceKind::Meters(63.0) }), Some("G".to_string()));

//    g.add_node(n1.clone());
//    g.add_node(n2.clone());
//    g.add_node(n3.clone());
//    g.add_node(n4.clone());
//    g.add_node(n5.clone());
//    g.add_node(n6.clone());
//    g.add_node(n7.clone());

//    g.connect_all_nodes();


        let g = primer.replicate_all();

        let pk = ProblemKind::GreedyTour { graph_guid: g.get_graph_guid() };
        let mut solve = Solution::new(g);
        solve.solve(&pk);

        for g in solve.get_graph().get_sub_graphs().clone() {
            println!("{}", g.1.get_edge_distance());
            println!("Greedy Tour Graph: {}", g.1);
            if let Some(node) = g.1.get_nodes().iter().next() {
                if let Some(path) = g.1.get_ordered_path_for_node(node.0.clone()) {
                    for p in path {
                        let the_node = g.1.get_node(p.clone()).unwrap();
                        let the_degree = g.1.get_degree_of_node(the_node.clone().get_guid());
                        println!("{}({}) - {}", p, the_degree, the_node);
                    }
                }
            }
        }

        let next = primer.replicate_all();
        let next_pk = ProblemKind::MinimumSpanningTree { graph_guid: next.get_graph_guid() };
        let mut next_solve = Solution::new(next);
        next_solve.solve(&next_pk);

        for g in next_solve.get_graph().get_sub_graphs().clone() {
            println!("{}", g.1.get_edge_distance());

            println!("MST Graph: {}", g.1);
            if let Some(node) = g.1.get_nodes().iter().next() {
                if let Some(path) = g.1.get_tree_for_node(node.0.clone()) {
                    for p in path {
                        let the_node = g.1.get_node(p.clone()).unwrap();
                        let the_degree = g.1.get_degree_of_node(the_node.clone().get_guid());
                        println!("{}({}) - {}", p, the_degree, the_node);
                    }
                }
            }
        }
    }
}
*/
/*
mod distance_tests {
    use ygg::metrics::uom;
    //    use ygg::metrics::uom::Si;
    use ygg::metrics::formulas;

    fn main_dorngus() {
        let d1 = uom::EQUATORIAL_RADIUS_OF_EARTH.to_miles();
        //let d1 = uom::DistanceKind::Miles(3963.0).to_kilometers();
        println!("Meters: {:?}", d1);

        let a = uom::PositionKind::ThreeDimensionEuclidean {
            x: uom::DistanceKind::Inches((12.0 * 5280.0) * 0.0),
            y: uom::DistanceKind::Feet((5280.0) * 3.0),
            z: uom::DistanceKind::Miles((1.0) * 4.0),
        };

        let b = uom::PositionKind::ThreeDimensionEuclidean {
            x: uom::DistanceKind::Feet(0.0),
            y: uom::DistanceKind::Inches(0.0),
            z: uom::DistanceKind::Miles(0.0),
        };

        let d = formulas::distance_between_two_points(&a, &b).to_miles();
        println!("{:?} to {:?} is {:?}", a, b, d);
    }
}
*/
/*
mod some_other_test {
    use std::env;
    use std::io;
    use std::rc::Rc;
    use ygg::graph_elements::graph;
    use ygg::graph_elements::node;
    use ygg::metrics::uom;
//    use ygg::solutions::Solution;
//    use ygg::solutions::ProblemKind;

    fn main_shoobert() {
        let args: Vec<String> = env::args().collect();
        let graph_size: i32;
        match args.get(1) {
            Some(x) => match x.parse::<i32>() {
                Ok(i) => {
                    graph_size = i;
                }
                Err(_) => {
                    println!("Not an integer.");
                    return;
                }
            },
            None => {
                println!("Graph size not provided.");
                return;
            }
        }


        let mut g = graph::Graph::new();

        let mut e1 = 0.0;
        let mut e2 = 0.0;
        let mut e3 = 0.0;
        let mut count = 0;
        for _number in { 0..graph_size } {
            let node = node::Node::new(
                Some(uom::PositionKind::ThreeDimensionEuclidean {
                    x: uom::DistanceKind::Feet(e1),
                    y: uom::DistanceKind::Feet(e2),
                    z: uom::DistanceKind::Feet(e3),
                }),
                Some(format!("Node {}", count)));
            g.add_node(Rc::clone(&node));
            e1 += 2.0;
            e2 += 4.0;
            e3 += 6.0;
            count += 1;
        }

        g.connect_all_nodes();

        match graph_size <= 50 {
            true => {
                println!("{}", g);
            }
            _ => (),
        }

        let mut guess = String::new();
        println!("done, press any key to end program.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    }
}
*/

