extern crate rand;

use rand::Rng;
use std::fmt;
use std::rc::Rc;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Guid {
    oct_0: u8,
    oct_1: u8,
    oct_2: u8,
    oct_3: u8,
    oct_4: u8,
    oct_5: u8,
    oct_6: u8,
    oct_7: u8,
    oct_8: u8,
    oct_9: u8,
    oct_10: u8,
    oct_11: u8,
    oct_12: u8,
    oct_13: u8,
    oct_14: u8,
    oct_15: u8,
}

impl Guid {
    pub fn new() -> Rc<Guid> {
        let mut rng = rand::thread_rng();
        Rc::new(Guid {
            oct_0: rng.gen_range(0, 127),
            oct_1: rng.gen_range(0, 127),
            oct_2: rng.gen_range(0, 127),
            oct_3: rng.gen_range(0, 127),
            oct_4: rng.gen_range(0, 127),
            oct_5: rng.gen_range(0, 127),
            oct_6: rng.gen_range(0, 127),
            oct_7: rng.gen_range(0, 127),
            oct_8: rng.gen_range(0, 127),
            oct_9: rng.gen_range(0, 127),
            oct_10: rng.gen_range(0, 127),
            oct_11: rng.gen_range(0, 127),
            oct_12: rng.gen_range(0, 127),
            oct_13: rng.gen_range(0, 127),
            oct_14: rng.gen_range(0, 127),
            oct_15: rng.gen_range(0, 127),
        })
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2X?}{:0>2X?}{:0>2X?}{:0>2X?}-{:0>2X?}{:0>2X?}-{:0>2X?}{:0>2X?}-{:0>2X?}{:0>2X?}-{:0>2X?}{:0>2X?}{:0>2X?}{:0>2X?}{:0>2X?}{:0>2X?}"
               , self.oct_0
               , self.oct_1
               , self.oct_2
               , self.oct_3
               , self.oct_4
               , self.oct_5
               , self.oct_6
               , self.oct_7
               , self.oct_8
               , self.oct_9
               , self.oct_10
               , self.oct_11
               , self.oct_12
               , self.oct_13
               , self.oct_14
               , self.oct_15
        )
    }
}

