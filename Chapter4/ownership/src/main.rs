fn main() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let _t = s.clone();
    let _u = s.clone();
    let nemoto:f64 = 0.5;
    let hasaki:f64 = nemoto;
    let hasa2i:f64 = nemoto;
    println!("{}", nemoto);
    println!("{}", hasaki);
    println!("{}", hasa2i);
    {
        let point = Box::new((0.625, 0.5));
        let label = format!("{:?}", point); 
        assert_eq!(label, "(0.625, 0.5)");
    }
    println!("Hello, world!");
    
    struct Person { name: Option<String>, birth: i32 }

    let mut composers = Vec::new();
    composers.push(Person { name: Some("Palestrina".to_string()),
                            birth: 1525 });
    // let first_name = std::mem::replace(&mut composers[0].name, None);
    let first_name = composers[0].name.take();
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);
    

    // composers.push(Person { name: "Dowland".to_string(),
    //                         birth: 1563 });
    // composers.push(Person { name: "Lully".to_string(),
    //                         birth: 1632 });
    // for composers in &composers {
    //     println!("{}, born {}", composers.name, composers.birth);
    // }

    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }
    
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");

    let second = v.swap_remove(1);
    assert_eq!(second, "102");
 
    let  third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    assert_eq!(v, vec!["101", "104", "substitute"]);
}
