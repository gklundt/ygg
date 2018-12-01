#![feature(core_intrinsics)]
extern crate ygg;


pub fn main() {
    test_boxed::main();
}

// Boxed Generics
mod test_boxed {
    trait Noun { fn hooplah(); }

    #[derive(Debug)]
    enum ThingType { Car, House, Rock }

    impl Noun for ThingType { fn hooplah() { println!("I'm a thing!!") } }

    #[derive(Debug)]
    enum PlaceType { NewYork, Home, Denton }

    impl Noun for PlaceType { fn hooplah() { println!("I'm a place!!") } }

    #[derive(Debug)]
    enum PersonType { Dad, Mom, Granny }

    impl Noun for PersonType { fn hooplah() { println!("I'm a person!!") } }

    pub trait Thingy { fn ima(&self); }

    #[derive(Debug)]
    pub struct Thing<T: Noun> { name: String, value: String, noun: Box<T> }

    impl<T: Noun> Thingy for Thing<T> { fn ima(&self) { println!("I'm a {:?}", self.noun) } }

    pub trait Thingies<N: Noun> { fn a(&mut self, noun: N); }

    pub struct Things<T: Thingy> { things: Vec<Box<T>> }

    impl<N: Noun, T: Thingy> Thingies for Things<T> { fn a(&mut self, noun: N) { self.things.push(noun); } }


    pub fn main() {
        let t1 = Thing { name: "hi".to_string(), value: "there".to_string(), noun: Box::new(PlaceType::Denton) };
        let t2 = Thing { name: "hi".to_string(), value: "there".to_string(), noun: Box::new(PersonType::Dad) };
        let t3 = Thing { name: "hi".to_string(), value: "there".to_string(), noun: Box::new(ThingType::Car) };

        println!("Thing: {:?}", t1);
        println!("Thing: {:?}", t2);
        println!("Thing: {:?}", t3);

//        let mut t4 = Things { things: Vec::new() };
//        t4.a(t1);
//        t4.a(t2);
//        t4.a(t3);
    }
}

// Traveler
/*
mod traveler_sandbox {
    use ygg::metrics::uom::distance;
    use ygg::metrics::uom::time;
    use ygg::heuristics::resources::Resource;

    //    use ygg::heuristics::travelers::Traveler;
//    use std::rc::Rc;
    use ygg::metrics::uom::UnitOfMeasureValueKind;
    use std::any::Any;

    pub fn main() {
        let distance_resource_name = "Distance".to_string();
        let distance_resource_min = distance::DistanceKind::Miles(0.0);
        let distance_resource_max = distance::DistanceKind::Miles(1000.0);
        let distance_resource_start = distance::DistanceKind::Miles(0.0);


        let time_resource_name = "Time".to_string();
        let time_resource_min = time::TimeKind::Hours(0.0);
        let time_resource_max = time::TimeKind::Hours(8.0);
        let time_resource_start = time::TimeKind::Hours(0.0);


        let distance_resource =
            Resource::new(
                distance_resource_name,
                distance_resource_min,
                distance_resource_max,
                distance_resource_start,
            );

        let time_resource =
            Resource::new(
                time_resource_name,
                time_resource_min,
                time_resource_max,
                time_resource_start,
            );


        let mut x = [
            &distance_resource,
            &time_resource,
        ];

        println!("{:?}\n{:?}", time_resource.clone().get_current_value(), distance_resource.clone().get_current_value());


//        let mut v:Vec<Box<UnitOfMeasureValueKind>>= Vec::new();
//        v.push(Box::new(time_resource));
//        v.push(Box::new(distance_resource));

//        let traveler_name = "Fred".to_string();
//        let mut traveler = Traveler::new(traveler_name);
//        traveler.push_resource(Rc::new(distance_resource));
//        traveler.push_resource(Rc::new(time_resource));
    }
}
*/

// Macro
/*
mod macro_sandbox {
    use ygg::metrics::uom::DistanceKind;
    use ygg::metrics::uom::VolumeKind;
    macro_rules! o_O { ($x:expr) => {
    println!("{:?}", $x)
    }}



    pub fn main() {

        let d = DistanceKind::Meters(1.2);

        match d {
            DistanceKind::Meters(x) => { println!("I got an {}", x)},
            _ => {println!("nada")},
        }

        let v = VolumeKind::CubicMeters(3.3);
        o_O!(d);
        o_O!(v);

    }
}
*/

// Resource
/*
mod resource_sandbox {
    use ygg::heuristics::resources::Resource;
    use ygg::metrics::uom::distance::DistanceKind;




    pub fn main() {

        let name = "Hiya".to_string();
        let min = DistanceKind::Miles(0.0);
        let max = DistanceKind::Miles(5.0);
        let start = DistanceKind::Feet(0.0);

        let mut r = Resource::new(name, min, max, start);

        println!("{:?}", r);

        let x = r.get_min();
        let w = r.get_max();
        let y = r.get_current_value();
        println!("capacity => {:?} to {:?}", x, w);
        println!("current value => {:?}", y);
        println!("diffs => min: {:?}, max: {:?}\n", y.clone() - x, w - y.clone());

        println!("{:?}", r);

        r.push_modification(DistanceKind::Feet(5.0));
        let x = r.get_min();
        let w = r.get_max();
        let y = r.get_current_value();
        println!("capacity => {:?} to {:?}", x, w);
        println!("current value => {:?}", y);
        println!("diffs => min: {:?}, max: {:?}\n", y.clone() - x, w - y.clone());

        println!("{:?}", r);

        r.push_modification(DistanceKind::Inches(-60.0));
        let x = r.get_min();
        let w = r.get_max();
        let y = r.get_current_value();
        println!("capacity => {:?} to {:?}", x, w);
        println!("current value => {:?}", y);
        println!("diffs => min: {:?}, max: {:?}\n", y.clone() - x, w - y.clone());

        println!("{:?}", r);
    }
}
*/

// Heuristics Constraints
/*
mod test_heuristics_constraints_sandbox {
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
*/

// Graph Kruskal's Spanning Tree
/*
mod test_graph_kruskal_spanning_tree {
    extern crate ygg;

    //use rand::{thread_rng, Rng};
    use ygg::graph_elements::graph::Graph;
    use ygg::graph_elements::node::Node;
    use ygg::metrics::uom::distance::DistanceKind;
    use ygg::metrics::uom::position::PositionKind;
    use ygg::solutions::ProblemKind;
    use ygg::solutions::Solution;
    use rand::thread_rng;
    use rand::Rng;


    pub fn main() {
        let mut primer = Graph::new();

        for i in 0..51 {
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

        primer.connect_all_nodes();
        //println!("{}", primer);
        println!("{}", primer.get_edge_distance());


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

// Graph Spanning Tree vs. Constrained Tree to path
/*
mod test_path_versus_tree {
    extern crate ygg;

    //use rand::{thread_rng, Rng};
    use ygg::graph_elements::graph::Graph;
    use ygg::graph_elements::node::Node;
    use ygg::metrics::uom::distance::DistanceKind;
    use ygg::metrics::uom::position::PositionKind;
    use ygg::solutions::ProblemKind;
    use ygg::solutions::Solution;
    use rand::thread_rng;
    use rand::Rng;
    use std::rc::Rc;


    pub fn main() {
        let mut primer = Graph::new();


        let n1: Rc<Node> = Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(80.0), y: DistanceKind::Meters(90.0) }), Some("A".to_string()));
        let n2: Rc<Node> = Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(60.0), y: DistanceKind::Meters(55.0) }), Some("B".to_string()));
        let n3: Rc<Node> = Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(45.0), y: DistanceKind::Meters(18.0) }), Some("C".to_string()));
        let n4: Rc<Node> = Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(38.0), y: DistanceKind::Meters(30.0) }), Some("D".to_string()));

        primer.add_node(n1.clone());
        primer.add_node(n2.clone());
        primer.add_node(n3.clone());
        primer.add_node(n4.clone());

        primer.add_connected_nodes((n1.clone(), n2.clone()));
        primer.add_connected_nodes((n3.clone(), n2.clone()));
        primer.add_connected_nodes((n4.clone(), n3.clone()));
        primer.add_connected_nodes((n4.clone(), n3.clone()));

        println!("Graph: \n{}", primer);

        println!("Tree: \n");
        if let Some(thing) = primer.get_path_for_node(n2.get_guid().clone()) {
            for t in thing {
                println!("\tTree Member: {}", t)
            }
        } else { println!("Not a Path"); }
    }
}
*/

// Graph Fully Connected
/*
mod test_fully_connected_graph {
    extern crate ygg;

    //use rand::{thread_rng, Rng};
    use ygg::graph_elements::graph::Graph;
    use ygg::graph_elements::node::Node;
    use ygg::metrics::uom::distance::DistanceKind;
    use ygg::metrics::uom::position::PositionKind;
    use ygg::solutions::ProblemKind;
    use ygg::solutions::Solution;
    use rand::thread_rng;
    use rand::Rng;
    use std::rc::Rc;


    pub fn main() {
        let mut primer = Graph::new();

        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(85.21773885774718), y: DistanceKind::Meters(62.173474906841335) }), Some("Node: 0".to_string())));
        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(85.3180434700734), y: DistanceKind::Meters(46.95934591522585) }), Some("Node: 1".to_string())));
        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(19.26007772031716), y: DistanceKind::Meters(98.86595581386) }), Some("Node: 2".to_string())));
        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(15.357552133032012), y: DistanceKind::Meters(10.593154195440874) }), Some("Node: 3".to_string())));
        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(50.97348747875345), y: DistanceKind::Meters(32.653332993210746) }), Some("Node: 4".to_string())));
        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(61.107372713944216), y: DistanceKind::Meters(7.524174645844415) }), Some("Node: 5".to_string())));
        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(49.923418279681634), y: DistanceKind::Meters(91.09706074700215) }), Some("Node: 6".to_string())));
        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(1.3503651113306159), y: DistanceKind::Meters(49.45295079739459) }), Some("Node: 7".to_string())));
        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(2.61376646950271), y: DistanceKind::Meters(48.17944662297873) }), Some("Node: 8".to_string())));
        primer.add_node(Node::new(Some(PositionKind::TwoDimensionEuclidean { x: DistanceKind::Meters(17.924954051784308), y: DistanceKind::Meters(39.42091381167005) }), Some("Node: 9".to_string())));


        primer.connect_all_nodes();
        println!("{}", primer);
        println!("{}", primer.get_edge_distance());
    }
}
*/

// Distance Tests
/*
mod distance_tests {
    use ygg::metrics::uom::position;
    use ygg::metrics::uom::distance;
    //    use ygg::metrics::uom::Si;
    use ygg::metrics::formulas;

    pub fn main() {
        let d1 = distance::EQUATORIAL_RADIUS_OF_EARTH.as_miles().clone();
        //let d1 = uom::DistanceKind::Miles(3963.0).to_kilometers();
        println!("Meters: {:?}", d1);

        let a = position::PositionKind::ThreeDimensionEuclidean {
            x: distance::DistanceKind::Inches((12.0 * 5280.0) * 0.0),
            y: distance::DistanceKind::Feet((5280.0) * 3.0),
            z: distance::DistanceKind::Miles((1.0) * 4.0),
        };

        let b = position::PositionKind::ThreeDimensionEuclidean {
            x: distance::DistanceKind::Feet(0.0),
            y: distance::DistanceKind::Inches(0.0),
            z: distance::DistanceKind::Miles(0.0),
        };

        let d = formulas::distance_between_two_points(a, b).as_miles().clone();
        println!("{:?} to {:?} is {:?}", a, b, d);
    }
}
*/

// Graph testing 3D spatial
/*
mod test_three_dimensional_graph {
    use std::env;
    use std::io;
    use std::rc::Rc;
    use ygg::graph_elements::graph;
    use ygg::graph_elements::node;
    use ygg::metrics::uom::distance;
    use ygg::metrics::uom::position;

    pub fn main() {
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
                graph_size = 4;
                println!("Graph size not provided.");
                //return;
            }
        }


        let mut g = graph::Graph::new();

        let mut e1 = 0.0;
        let mut e2 = 0.0;
        let mut e3 = 0.0;
        let mut count = 0;
        for _number in { 0..graph_size } {
            let node = node::Node::new(
                Some(position::PositionKind::ThreeDimensionEuclidean {
                    x: distance::DistanceKind::Feet(e1),
                    y: distance::DistanceKind::Feet(e2),
                    z: distance::DistanceKind::Feet(e3),
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



