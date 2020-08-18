use std::str::FromStr;
use std::collections::HashMap;
use std::iter::Peekable;

// fn hoge(int: i32, float: f32) -> i32;

fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
    where I: Iterator<Item=char>
{
    let mut n = 0;
    loop {
        match tokens.peek() {
            Some(r) if r.is_digit(10) => {
                n = n * 10 + r.to_digit(10).unwrap();
            }
            _ => return n
        }
        tokens.next();
    }
}

struct Flanky(bool);

impl Iterator for Flanky {
    type Item = &'static str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 {
            self.0 = false;
            Some("totally the last item")
        } else {
            self.0 = true; // D'oh!
            None
        }
    }
}


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
    let message = "To: jimb\r\n\
                   From: superego <editor@oreilly.com>\r\n\
                   \r\n\
                   Did you get any writing time plotting fractals?\r\n\
                   When will you stop wasting time plotting fractals?\r\n";
    for header in message.lines().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }
    for body in message.lines()
        .skip_while(|l| !l.is_empty())
        .skip(1) {
        println!("{}", body);
    }
    let mut chars = "226153980,1766319049".chars().peekable();
    assert_eq!(parse_number(&mut chars), 226153980);
    // Look, `parse_number` didn't consume the comma! So we will.
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 1766319049);
    assert_eq!(chars.next(), None);
    let mut flanky = Flanky(true);
    assert_eq!(flanky.next(), Some("totally the last item"));
    assert_eq!(flanky.next(), None);
    assert_eq!(flanky.next(), Some("totally the last item"));
    let mut not_flanky = Flanky(true).fuse();
    assert_eq!(not_flanky.next(), Some("totally the last item"));
    assert_eq!(not_flanky.next(), None);
    assert_eq!(not_flanky.next(), None);

}

// fn _hoge(int: i32, float: f32) -> i32 {
//     let _ = float;
//     int
// }

