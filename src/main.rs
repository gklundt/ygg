extern crate ygg;

use rand::{thread_rng, Rng};
use ygg::graph_elements::graph::Graph;
use ygg::graph_elements::node::Node;
use ygg::metrics::uom::DistanceKind;
use ygg::metrics::uom::PositionKind;
use ygg::solutions::ProblemKind;
use ygg::solutions::Solution;
//use std::rc::Rc;

fn main() {
    let mut g = Graph::new();


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
//    g.add_connected_nodes((n3.clone(), n4.clone()));
//
//    println!("Graph: \n{}", g);
//
//    println!("Tree: \n");
//    if let Some(thing) = g.get_path_for_node(n1.get_guid().clone()) {
//        for t in thing {
//            println!("\t{}", t)
//        }
//    } else { println!("Not a path"); }


// Test for Shortest Tour Greedy
    for i in 0..500 {
        let name = format!("Node: {}", i);

        g.add_node(
            Node::new(
                Some(
                    PositionKind::TwoDimensionEuclidean {
                        x: DistanceKind::Meters(thread_rng().gen_range(1.0, 1000.0)),
                        y: DistanceKind::Meters(thread_rng().gen_range(1.0, 1000.0)),
                    }),
                Some(name),
            )
        );
    }


    g.connect_all_nodes();

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

    //println!("{}", g);


    let pk = ProblemKind::ShortestTour { graph_guid: g.get_guid() };
    let mut solve = Solution::new(g);


    solve.solve(&pk);

    // println!("Original Graph: {}", solve.get_graph());

    for g in solve.get_graph().get_sub_graphs().clone() {
        // println!("The Minimum Spanning Tree: {}", g.1);
        if let Some(node) = g.1.get_nodes().iter().next() {
            if let Some(path) = g.1.get_path_for_node(node.0.clone()) {
                for p in path {
                    let the_node = g.1.get_node(p.clone()).unwrap();
                    let the_degree = g.1.get_degree(the_node.clone().get_guid());
                    println!("{}({}) - {}", p, the_degree, the_node );
                }
            }
        }
    }
}


//use ygg::metrics::uom;
//use ygg::metrics::uom::Si;
//use ygg::metrics::formulas;
//
//fn main() {
//    let d1 = uom::EQUATORIAL_RADIUS_OF_EARTH.to_miles();
//    //let d1 = uom::DistanceKind::Miles(3963.0).to_kilometers();
//    println!("Meters: {:?}", d1);
//
//    let a = uom::PositionKind::ThreeDimensionEuclidean {
//        x: uom::DistanceKind::Inches((12.0 * 5280.0) * 0.0),
//        y: uom::DistanceKind::Feet((5280.0) * 3.0),
//        z: uom::DistanceKind::Miles((1.0) * 4.0),
//    };
//
//    let b = uom::PositionKind::ThreeDimensionEuclidean {
//        x: uom::DistanceKind::Feet(0.0),
//        y: uom::DistanceKind::Inches(0.0),
//        z: uom::DistanceKind::Miles(0.0),
//    };
//
//    let d = formulas::distance_between_two_points(&a, &b).to_miles();
//    println!("{:?} to {:?} is {:?}", a, b, d);
//}

//use std::env;
//use std::io;
//use std::rc::Rc;
//use ygg::graph_elements::graph;
//use ygg::graph_elements::node;
//use ygg::metrics::uom;
//use ygg::solutions::Solution;
//use ygg::solutions::ProblemKind;
//
//fn main() {
//    let args: Vec<String> = env::args().collect();
//    let graph_size: i32;
//    match args.get(1) {
//        Some(x) => match x.parse::<i32>() {
//            Ok(i) => {
//                graph_size = i;
//            }
//            Err(_) => {
//                println!("Not an integer.");
//                return;
//            }
//        },
//        None => {
//            println!("Graph size not provided.");
//            return;
//        }
//    }
//
//
//
//
//    let mut g = graph::Graph::new();
//
//    let mut e1 = 0.0;
//    let mut e2 = 0.0;
//    let mut e3 = 0.0;
//    let mut count = 0;
//    for _number in { 0..graph_size } {
//        let node = node::Node::new(
//            Some(uom::PositionKind::ThreeDimensionEuclidean {
//                x: uom::DistanceKind::Feet(e1),
//                y: uom::DistanceKind::Feet(e2),
//                z: uom::DistanceKind::Feet(e3),
//            }),
//            Some(format!("Node {}", count)));
//        g.add_node(Rc::clone(&node));
//        e1 += 2.0;
//        e2 += 4.0;
//        e3 += 6.0;
//        count += 1;
//    }
//
//    g.connect_all_nodes();
//
//    match graph_size <= 50 {
//        true => {
//            println!("{}", g);
//        }
//        _ => (),
//    }
////    let mut nvec = vec![];
////
////    for n in g.get_nodes() {
////        nvec.push(n.1);
////    }
////
////    g.remove_node_connection((Rc::clone(&nvec[0]), Rc::clone(&nvec[1])));
////
////    match graph_size <= 10 {
////        true => {
////            println!("{}", g);
////        }
////        _ => (),
////    }
////
////
//    let mut guess = String::new();
//    println!("done, press any key to end program.");
//    io::stdin()
//        .read_line(&mut guess)
//        .expect("Failed to read line");
//}
