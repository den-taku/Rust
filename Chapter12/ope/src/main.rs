use std::ops::{Neg, Add, AddAssign};

#[derive(Copy, Clone)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T, O> Neg for Complex<T>
    where T: Neg<Output=O>
{
    type Output = Complex<O>;
    
    fn neg(self) -> Complex<O> {
        Complex{ re: -self.re, im: -self.im }
    }
}

impl<L, R, O> Add<Complex<R>> for Complex<L> 
    where L: Add<R, Output=O>
{
    type Output = Complex<O>;
    
    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

fn main() {
    assert_eq!(4.125f32.add(5.75), 9.875);
    assert_eq!(10.add(20), 10 + 20);

    let mut a = Complex{ re: 3f32, im: 4f32 }; 
    let b = Complex{ re: 9f32, im: 8f32 }; 
    println!("( {} + {}i ) + ( {} + {}i ) = {} + {}i", 
        a.re, a.im, b.re, b.im, 
        (a + b).re, <Complex<f32> as Add<Complex<f32>>>::add(a, b).im);
    println!("-({} + {}i) = {} + {}i",
        a.re, a.im, (-a).re, <Complex<f32> as Neg>::neg(a).im);
    println!("({} + {}i) += ({} + {}i)", a.re, a.im, b.re, b.im); 
    <Complex<f32> as AddAssign<Complex<f32>>>::add_assign(&mut a, b);
    println!(" = {} + {}i", a.re, a.im);    

    let mut s:&str = "hoge";
    println!("{}", s);
    s = "hogehoge";    
    println!("{}", s);

    println!("Hello, world!");
}
