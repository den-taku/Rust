use std::collections::{HashMap, HashSet};
use std::io::prelude::*;

mod mimpl {
    // impl Cell {
    //     pub fn distande_from_origin(&self) -> f64 {
    //         f64::hypot(self.x, self.y)
    //     }
    // }

    pub const ROOM_TEMPERATURE: f64 = 20.0;
    pub static ROM_TEMPERATURE: f64 = 20.0;
}

mod parent {
    use std::collections::HashMap;
    mod child {
        use std::collections::HashMap;
        struct Num {
            map: HashMap<i32, i32>
        }
    }
}

fn main() {
    println!("Hello, world!");
}
