use rand::random;

/// std::ops::Mul, the trait for types that support `*`.
pub trait Mul<RHS=Self> {
    /// The resulting type after applying the `*` operator
    type Output;

    /// The method for the `*` operator
    fn mul(self, rhs: RHS) -> Self::Output;
}

fn main() {
    println!("Hage!");
    let x = random::<f64>();
    let y = random::<f64>();
    let b = random::<bool>();
    println!("{}", x);
    println!("{}", y);
    println!("{}", b);
    println!("Hello, world!");
}
