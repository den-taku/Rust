fn _triangle(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n+1 {
        sum  += i;
    }
    sum
}

fn triangle2(n: i32) -> i32 {
    (1..n+1).fold(0, |sum, item| sum + item)
}

fn main() {
    println!("sum: {}", triangle2(920));
    
    println!("There's:");
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];

    for element in &v {
        println!("{}", element);
    }
}
