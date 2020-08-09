fn f() {
    return;
}

fn type_of<T>(_: T) ->String {
    let a = std::any::type_name::<T>();
    a.to_string()
} 

fn main() {
    let a = { struct S {a: i32, b: i32} };
    println!("{}", type_of(a));
    let b = 12f32;
    let b = b;
    println!("{}", b);
    let c = if true {;};
    println!("{}", type_of(c));
    let d = if true { 3 } else { 5 };
    println!("{}", type_of(d));

    let code = 9; 
    match code {
        0 => println!("Ok!"),
        1 => println!("Wires Tangled"),
        2 => println!("User Sleep"),
        _ => println!("Unrecognized Error {}", code)
    }
 
    // let g = (let f = 32);
    // println!("{}", type_of(g));

    for i in 1..20 {
        println!("{}", i);
    }

    'a:
    for i in 1..6 {
        'b:
        for j in 1..6 {
            if i * j == 12 { continue 'a; } 
            println!("{}", i * j); 
            if i * j == 9 { break 'b; }
        }
    }
    let a = {
        for i in 1..20 {
            if i == 18 {
                return ();
            }
        }
    };
    loop {
        let mut i = 0;
        i += 1;
        println!("{}", i);
        if i == 30 {
            return;
        }
    }
}
