use std::ffi::OsStr;
use std::path::Path;

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

    let v = vec![4, 20, 12, 8, 6];
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&12));
    assert_eq!(iterator.next(), Some(&8));
    assert_eq!(iterator.next(), Some(&6));
    assert_eq!(iterator.next(), None);

    let path = Path::new("C:/Users/JimB/Downloads/Fedora.iso");
    let mut iterator = path.iter();
    assert_eq!(iterator.next(), Some(OsStr::new("C:")));
    assert_eq!(iterator.next(), Some(OsStr::new("Users")));
}
