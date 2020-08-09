#[test]
fn math_works() {
    let x: i32 = 1;
    debug_assert!(x.is_positive());
    debug_assert_eq!(x + 1, 2);
} 

// #[test]
// #[should_panic(expected="devide by zero")]
// fn test_devide_by_zero_error() {
//     1/0;
// }

#[cfg(test)] // include this module only when testing
mod tests {
    fn roughly_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn trig_works() {
        use std::f64::consts::PI;
        assert!(roughly_equal(PI.sin(), 0.0));
    }
}

fn main() {
    println!("Hello, world!");
}
