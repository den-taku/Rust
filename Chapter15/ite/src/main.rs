fn triangle(n: i32) -> i32 {
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
    println!("{}", triangle2(920));
}
