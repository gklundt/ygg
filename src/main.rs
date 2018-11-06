extern crate ygg;

use std::env;
use std::io;
use std::rc::Rc;
use ygg::graph_elements::graph;
use ygg::graph_elements::node;
use ygg::graph_elements::location;
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

    for _number in { 0..graph_size } {
        let node = node::Node::new(
            Some(location::LocationKind::TwoDimensionGeo {
                lat: uom::AngularKind::Degrees(37.9),
                lng: uom::AngularKind::Degrees(-97.3434),
            }),
            None);
        g.add_node(Rc::clone(&node));
    }

    g.connect_all_nodes();

    match graph_size <= 10 {
        true => {
            println!("{}", g);
        }
        _ => (),
    }
    let mut nvec = vec![];

    for n in g.get_nodes() {
        nvec.push(n.1);
    }

    g.remove_node_connection((Rc::clone(&nvec[0]), Rc::clone(&nvec[1])));

    match graph_size <= 10 {
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
