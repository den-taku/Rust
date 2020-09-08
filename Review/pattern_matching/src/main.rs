fn main() {
    let a = Some(65);
    match a {
        Some(num) => {
            if num == 64 {
                println!("64!!!!");
            } else {
                println!("hoge");
            }
        }
        None => ()
    }
    // let v = vec![1, 2, 3];
    // v = match v {
    //     vec![1, 2] => vec![1, 2, 3],
    //     vec![1, 2, 3] => vec![1, 2, 3, 4],
    //     _ => vec![]
    // };
    match a {
        ref _option => println!("hoge"),
    }
    match &a {
        &None => (),
        &Some(num) => println!("{}", num)
    }
    match a {
        Some(num) => {
            match num {
                32 | 45 | 64 => println!("nice!"),
                _ => ()
            }
        }
        None => ()
    }
    match a {
        Some(num) if num == 34 => (),
        Some(_num) => println!("()"),
        None => ()
    }
    match a {
        some @ Some(..) => println!("{:#?}", some),
        _ => ()
    }
    // let v = vec![1, 2, 3];
    // match &v {
    //     vec @ [1, 2, 3].as_vec() => ()
    // }
    match a {
        _ => println!("")
    }
    println!("Hello, world!");
}
