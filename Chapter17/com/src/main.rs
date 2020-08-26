use std::fmt;

struct Complex { r: f64, i: f64 }

// impl fmt::Display for Complex {
//     fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
//         let i_sign = if self.i < 0.0 { '-' } else { '+' };
//         write!(dest, "{} {} {}i", self.r, i_sign, f64::abs(self.i))
//     }
// }

impl fmt::Display for Complex {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let (r, i) = (self.r, self.i);
        if dest.alternate() {
            let abs = f64::sqrt(r * r + i + i);
            let angle = f64::atan2(i, r) / std::f64::consts::PI * 180.0;
            write!(dest, "{} ∠ {}°", abs, angle)
        } else {
            let i_sign = if i < 0.0 { '-' } else { '+' };
            write!(dest, "{} {} {}i", r, i_sign, f64::abs(i))
        }
    }
}

fn main() {
    println!("Hello, world!");
    let one_twenty = Complex { r: -0.5, i: 0.866 };
    println!("{}", one_twenty);

    let two_forty = Complex { r: -0.5, i: -0.866};
    println!("{}", two_forty);

    let ninety = Complex { r: 0.0, i: 2.0 };
    println!("{}", ninety);
    println!("{:#}", ninety);
}
