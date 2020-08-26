use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Complex { r: f64, i: f64 }

fn main() {
    println!("{:.3}μs: relocated {} at {:#x} to {:#x}, {} bytes",
        0.84391, "object",
        140737488346304_usize, 6299664_usize, 64);
    println!("{{a, c}} ⊂ {{a, b, c}}");
    println!("{:*<4}", "th\u{e9}");
    println!("{:*<4}", "th\u{301}");
    println!("{:=^12}", 1234);

    let mut map = HashMap::new();
    map.insert("Portland", (45.5237606,-122.6819273));
    map.insert("Taipei", (25.0375167,121.5637));
    println!("{:#?}", map);

    let third = Complex{ r: -0.5, i: f64::sqrt(0.75) };
    println!("{:?}", third);

    let original = Rc::new("mazurka".to_string());    
    let cloned = original.clone();
    let impostor = Rc::new("mazurka".to_string());
    println!("text:     {}, {}, {}", original, cloned, impostor);
    println!("pointers: {:p}, {:p}, {:p}", original, cloned, impostor);

    println!("{mode} {2} {} {}",
        "people", "eater", "purple", mode="flying");
}
