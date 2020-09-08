fn main() {
    let mut v = vec![1, 2, 3];
    let a = [1, 3, 5];
    println!("{:?}", v.iter_mut());
    println!("{:?}", v);
    // println!("{:?}", v.into_iter());
    println!("{:?}", (&v).iter()
                      .map(|n| n + 1)
                      .filter(|n| *n > 1)
                      .fuse()
                      .rev()
                      .fold(0, |n, i| n + i)
    );
}
