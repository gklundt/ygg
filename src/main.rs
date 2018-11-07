extern crate ygg;
//
//use ygg::metrics::uom;
//use ygg::metrics::uom::Si;
//use ygg::metrics::formulas;
//
//fn main() {
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

use std::env;
use std::io;
use std::rc::Rc;
use ygg::graph_elements::graph;
use ygg::graph_elements::node;
use ygg::metrics::uom;


fn main() {
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

    let mut e1= 0.0;
    let mut e2 = 0.0;
    let mut e3 = 0.0;
    let mut count = 0;
    for _number in { 0..graph_size } {
        let node = node::Node::new(
            Some(uom::PositionKind::ThreeDimensionEuclidean  {
                x: uom::DistanceKind::Feet(e1),
                y: uom::DistanceKind::Feet(e2),
                z: uom::DistanceKind::Feet(e3),
            }),
            Some(format!("Node {}",count)));
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
//    let mut nvec = vec![];
//
//    for n in g.get_nodes() {
//        nvec.push(n.1);
//    }
//
//    g.remove_node_connection((Rc::clone(&nvec[0]), Rc::clone(&nvec[1])));
//
//    match graph_size <= 10 {
//        true => {
//            println!("{}", g);
//        }
//        _ => (),
//    }
//
//
    let mut guess = String::new();
    println!("done, press any key to end program.");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
}
