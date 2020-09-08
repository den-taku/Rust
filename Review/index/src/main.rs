fn main() {
    let mut array = [0,1,2,3,4,5,6,7,8,9];
    array[2] = 20;
    println!("{:#?}", array);
    let mut a = 10;
    let mut b = &a;
    b = &20;
    println!("{}", a);
}
