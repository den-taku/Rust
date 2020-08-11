use std::ops::Add;

struct Complex<T> {
    re: T,
    im: T
}

impl<L, R, O> Add<Complex<R>> for Complex<L> 
    where L: Add<R, Output=O>
{
    type Output = Complex<O>;
    
    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

fn main() {
    assert_eq!(4.125f32.add(5.75), 9.875);
    assert_eq!(10.add(20), 10 + 20);

    println!("Hello, world!");
}
