use std::str::FromStr;
use std::collections::HashMap;

// fn hoge(int: i32, float: f32) -> i32;

fn main() {
    let text = "  ponies  \n    giraffes\niguanas  \nsquid".to_string();
    let v: Vec<&str> = text.lines()
        .map(str::trim)
        .collect();
    for element in &v {
        println!("{}", *element);
    }
    let text = "  ponies  \n    giraffes\niguanas  \nsquid".to_string();
    let v: Vec<&str> = text.lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    for element in &v {
        println!("{}", *element);
    }
    let text = "1\nfrond .25 289\n3.1415 estuary\n";
    for number in text.split_whitespace()
                      .filter_map(|w| f64::from_str(w).ok()) {
        println!("{:4.2}", number.sqrt());
    }
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["Sao Paulo", "Brasilia"]);
    major_cities.insert("Kenya", vec!["Nairobe", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);
    let countries = ["Japan", "Brazil", "Kenya"];
    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }
    let iter = (0..10)
        .scan(0, |sum, item| {
            *sum += item;
            if *sum > 10 {
                None
            } else {
                Some(item * item)
            }
        });
    for element in &iter.collect::<Vec<i32>>() {
        println!("{}", element);
    }
}

// fn _hoge(int: i32, float: f32) -> i32 {
//     let _ = float;
//     int
// }

