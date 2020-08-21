use std::collections::VecDeque;

fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}

fn main() {
    let mut v = VecDeque::from(vec![1, 2, 3, 4]);
    println!("{}", type_of(v.front_mut()));
}
