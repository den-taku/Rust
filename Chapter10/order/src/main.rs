// use self::Ordering::*;
use std::mem::size_of;

enum Ordering {
    Less    = 200,
    Equal   = 300,
    Greater = 400
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years
}

impl TimeUnit {
    /// Return the plural noun for this time unit.
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "munutes",
            TimeUnit::Hours   => "hours",
            TimeUnit::Days    => "days",
            TimeUnit::Months  => "months",
            TimeUnit::Years   => "years"
        }
    }

    /// Return the singular noun for this time unit.
    fn singular(self) -> &'static str {
        self.plural().trim_right_matches('s')
    }
}

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn ordering_from_u32(n: u32) -> Option<Ordering> {
    match n {
        200 => Some(Ordering::Less),
        300 => Some(Ordering::Equal),
        400 => Some(Ordering::Greater),
         _  => None
    }
}

fn main() {
    println!("Hello, world!");
    println!("{}", Ordering::Less as i32);
    println!("{}", size_of::<Ordering>());
    println!("{}", TimeUnit::Years.plural());
}

