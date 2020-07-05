fn type_of<T>(_: T) ->String {
    let a = std::any::type_name::<T>();
    a.to_string()
}

fn looploop () -> ! {
    loop {
        break;
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
    println!("{}", type_of(loop { ; }));
}
