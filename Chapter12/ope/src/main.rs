use std::ops::{Neg, Add, Sub, Mul, AddAssign};
use std::cmp::PartialEq;

#[derive(Copy, Clone, Debug, Eq)]
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

impl<L, R, O> Mul<Complex<R>> for Complex<L>
    where L: Copy + Mul<R, Output=O> + Add<R, Output=O>,
          R: Copy,
          O: Copy + Sub<O, Output=O> + Add<O, Output=O>
{
    type Output = Complex<O>;
  
    fn mul(self, rhs: Complex<R>) -> Self::Output {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re
        }
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

impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
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
    a += -b;
    println!("({} + {}i) == ({} + {}i) -> {}", 
        a.re, a.im, b.re, b.im, 
        <Complex<f32> as PartialEq<Complex<f32>>>::eq(&a, &b));
    println!("({} + {}i) * ({} + {}i) = {} + {}i",
        a.re, a.im, b.re, b.im,
        <Complex<f32> as Mul<Complex<f32>>>::mul(a, b).re, (a * b).im);

    let x = Complex{ re:5, im:2 };
    let y = Complex{ re:2, im:5 };
    // assert_eq!(x * y, Complex{ re:0, im:29 });
    let _ = x * y;

    let mut s:&str = "hoge";
    println!("{}", s);
    s = "hogehoge";    
    println!("{}", s);

    println!("0.0 / 0.0 == 0.0 / 0.0 -> {}", 0.0 / 0.0 == 0.0 / 0.0);

    println!("Hello, world!");
}
