/// A timestamp that has been deliberately rounded off, so our program
/// says "6 months ago" instead of "February 9, 2016, at 9:49 AM".
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32)
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

enum Shape {
    Sphere { center: f32, radius: f32 },
    Cuboid { corner1: f32, couner2: f32 }
}

enum RelationshipStatus {
    Single,
    InAReationship,
    ItsComplicated(Option<String>),
    ItsExtremelyComplicated {
        car: i32,
        cdr: u32
    }
}

fn main() {
    let four_score_and_seven_years_ago =
        RoughTime::InThePast(TimeUnit::Years, 4*20 + 7);
    let three_hours_from_now =
        RoughTime::InTheFuture(TimeUnit::Hours, 3);
    let unit_sphere = Shape::Sphere { center: 0.0, radius: 1.0 };
    println!("Hello, world!");
}
