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
}
