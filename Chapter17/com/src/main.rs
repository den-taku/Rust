use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;

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

fn loggin_enabled() -> bool { true }

fn write_log_entry(entry: std::fmt::Arguments) {
    if loggin_enabled() {
        // Keep things simple for now, and just
        // open the file every time.
        let mut log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("log-file-name")
            .expect("failed to open log file");

        log_file.write_fmt(entry)
            .expect("failed to write to log");
    }
}

macro_rules! log { // no ! needed ager name in macro deginitions
    ($format:tt, $($arg:expr), *) => (
        write_log_entry(format_args!($format, $($arg), *))
    )
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

    write_log_entry(format_args!("Hark! {:?}\n", 7893295));
    log!("0 day and night, but this is wondrous strange! {:?}\n", 3425632);
}
