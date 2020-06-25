fn type_of<T>(_: T) ->String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}

fn main() {
    println!("{}", (2_f64).sqrt());
    println!("{}", type_of(2.0));
    println!("{}", (2e0f32.sqrt()));
    println!("{}", (-1. / std::f32::INFINITY).is_sign_negative());
}
