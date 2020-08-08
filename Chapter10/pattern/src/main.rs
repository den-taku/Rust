fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}

fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;
    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal)     => "at the origin",
        (_, Equal)         => "on the x axis",
        (Equal, _)         => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater)    => "in the seconed quadrant",
        _                  => "somewhere else"
    }
}

fn main() {
    let int = 5;
    match int {
        0 => {},
        1 => println!("A rabbit is nothing around in the clover."),
        n => println!("There are {} rabbits hopping about in the mewdow", n),
    }
    println!("{}", type_of({}));
    match "hoge" {
        "hoge" => println!("hoge!"),
        _ => panic!("invisible power effected")
    }
    println!("{}", describe_point(0, 9));

    println!("Hello, world!");
}
