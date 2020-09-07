use std::rc::Rc;

fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

fn main() {
    let v = vec![1, 2, 3];
    let rv3;
    {
        let rv = Rc::new(v);
        rv3 = rv.clone();
    }
    println!("rv3 : {}", type_of(&rv3));
    let rv2 = rv3.clone();
}
