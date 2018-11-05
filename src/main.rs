extern crate ygg;
use std::env;
use std::io;
use std::rc::Rc;
use ygg::graph_elements::graph;
use ygg::graph_elements::node;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut graph_size = 1;
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
        let node = node::Node::new();
        g.add_node(Rc::clone(&node));
    }

    g.connect_all_nodes();
    
    match graph_size <= 10 {
        true => {println!("{}", g);},
        _ => (),
    } 

    let mut guess = String::new();
    println!("done, press any key to end program.");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
}
