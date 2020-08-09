fn type_of<T>(_: T) ->String {
    let a = std::any::type_name::<T>();
    a.to_string()
}

fn looploop () -> ! {
    loop {
    }
}

fn oppp () -> i32 {
    while true {
        if true { break; }
    }

    loop {
        if true {
            return 32;
        }
    }
}

fn main() {
    'a:
    loop {
        for j in 1..20{
            for i in 1..20 {
                println!("{}", i);
            }
            if j == 19 {
                break ;
            }
        }
        break;
    }
    
    while true {
        for i in 1..20 {
            for j in 1..20 {
                println!("{}", j);
            }
        }
        if true {
            break;
        }
    }
    println!("{}", 19023.23 % 100.0);
    println!("{}",std::f32::INFINITY as u8);

    let v = vec![1, 2, 3, 10];
    for vec in v {
        println!("{}", vec);
    }
}
