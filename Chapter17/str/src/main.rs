use std::str::FromStr;
use std::net::IpAddr;

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
}
