/// std::ops::Mul, the trait for types that support `*`.
pub trait Mul<RHS=Self> {
    /// The resulting type after applying the `*` operator
    type Output;

    /// The method for the `*` operator
    fn mul(self, rhs: RHS) -> Self::Output;
}

fn main() {
    println!("Hage!");

    println!("Hello, world!");
}
