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
    
    struct Person { name: String, birth: i32 }

    let mut composers = Vec::new();
    composers.push(Person { name: "Palestrina".to_string(),
                            birth: 1525 });
    composers.push(Person { name: "Dowland".to_string(),
                            birth: 1563 });
    composers.push(Person { name: "Lully".to_string(),
                            birth: 1632 });
    for composers in &composers {
        println!("{}, born {}", composers.name, composers.birth);
    }
}
