fn factorial(n: usize) -> usize {
    (1..n+1).fold(1, |a, b| a * b)
}

fn main() {
    let x = 10;
    let y = 10;
    let mut r = &x;

    if true { r = &y; }

    assert!(*r == 10 || *r == 20);
  
    struct Point { x: i32, y: i32 }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr; 

    println!("{}", (*rr).x);

    assert_eq!(rrr.y, 729); 

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(rx == ry);
    assert!(!std::ptr::eq(rx, ry));

    let R = &factorial(6);
    assert_eq!(R + &1009, 1729);

    let mut tmp1 = &1000;
    let tmp2 = &tmp1;
    println!("{}", tmp2);
    tmp1 = &900;
    println!("{}",tmp1);
    // println!("{}", tmp2);

    let mut v: Vec<i32> = vec![1, 2, 3, 4];
    let     s: &[i32] = &mut v;
    v.push(5);
    println!("Fu");
}
