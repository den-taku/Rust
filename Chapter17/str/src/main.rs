use std::str::FromStr;
use std::net::IpAddr;

fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

fn main() {
    if let Ok(num) = usize::from_str("3628800") {
        println!("usize: {}", num);
    }
    if let Ok(b) = bool::from_str("true") {
        println!("bool: {}", b);
    }

    let address = IpAddr::from_str("fe80::0000:3ea9:f4ff:fe34:7a50").expect("err");
    assert_eq!(address,
               IpAddr::from([0xfe80, 0, 0, 0, 0x3ea9, 0xf4ff, 0xfe34, 0x7a50]));
    let address = "fe80::0000:03a9:f4ff:fe34:7a50".parse::<IpAddr>().expect("err");

    println!("{}", format!("{}, wow", "doge"));
    println!("{}", format!("{}", true));
    println!("{}", format!("({:.3}, {:.3})", 0.5, f64::sqrt(3.0)/2.0));

    let formatted_addr: String = format!("{}", address);
    println!("{}", formatted_addr);
    println!("Type of \'formatted\' is {}.", type_of(&formatted_addr));
    println!("{}", formatted_addr.to_string());
    let addresses = vec![address, IpAddr::from_str("192.168.0.1").expect("err")];
    println!("{}", format!("{:?}", addresses));
}
