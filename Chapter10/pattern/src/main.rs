fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
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

    println!("Hello, world!");
}
