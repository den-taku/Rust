use module_system::module_a::module_a::*;
use module_system::module_b::module_b::*;

fn main() {
    let vec = Vector3d::new(3.0, 4.0, 5.0);
    hello();
    println!("{:?}", vec);
}
