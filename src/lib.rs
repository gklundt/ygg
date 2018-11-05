#![feature(test)]
#![feature(extern_crate_item_prelude)]
extern crate test;

pub mod graph_elements;
pub mod uuid;

use self::uuid::Guid;
use self::graph_elements::graph;
use self::graph_elements::node;
use std::rc::Rc;

pub fn bench_me() {
    let uuid = Guid::new();
    println!("{}", uuid);

    let mut g = graph::Graph::new();

    let n_first = node::Node::new();
    let n_second = node::Node::new();
    let n_third = node::Node::new();
    let n_fourth = node::Node::new();

    println!("{}", n_first);
    println!("{}", n_second);
    println!("{}", n_third);

    g.add_node(Rc::clone(&n_first));
    g.add_node(Rc::clone(&n_second));
    g.add_node(Rc::clone(&n_third));
    g.add_node(Rc::clone(&n_fourth));

    g.add_connection((Rc::clone(&n_first), Rc::clone(&n_second)));
    g.add_connection((Rc::clone(&n_first), Rc::clone(&n_second)));
    g.add_connection((Rc::clone(&n_first), Rc::clone(&n_second)));

    g.add_connection((Rc::clone(&n_second), Rc::clone(&n_first)));
    g.add_connection((Rc::clone(&n_second), Rc::clone(&n_first)));
    g.add_connection((Rc::clone(&n_second), Rc::clone(&n_first)));

    g.add_connection((Rc::clone(&n_second), Rc::clone(&n_second)));
    g.add_connection((Rc::clone(&n_second), Rc::clone(&n_second)));
    g.add_connection((Rc::clone(&n_second), Rc::clone(&n_second)));

    g.add_connection((Rc::clone(&n_first), Rc::clone(&n_first)));
    g.add_connection((Rc::clone(&n_first), Rc::clone(&n_first)));
    g.add_connection((Rc::clone(&n_first), Rc::clone(&n_first)));

    g.add_connection((Rc::clone(&n_first), Rc::clone(&n_third)));
    g.add_connection((Rc::clone(&n_third), Rc::clone(&n_second)));
    g.add_connection((Rc::clone(&n_fourth), Rc::clone(&n_second)));


    println!("{}", g);

    // let mut g = graph::Graph::new();
    // g.add_node(&n_first);
    // g.add_connection((&n_first, &n_second), &e);

    // g.add_node(n_second);
    // g.add_edge(e);

    // println!("{:?} {:?} {:?}", n_first, n_second, e);
    //println!("{:?}", g);
}



#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[bench]
    fn name(b: &mut Bencher) {
        b.iter(|| bench_me());
     }
}
