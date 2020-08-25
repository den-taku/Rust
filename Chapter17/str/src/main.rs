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

    let good_utf8: Vec<u8> = vec![0xe9, 0x8c, 0x86];
    if let Some(string) = String::from_utf8(good_utf8).ok() {
        println!("{}", string);
    }
    
    let bad_utf8: Vec<u8> = vec![0x9f, 0xf0, 0xa6, 0x80];
    let result = String::from_utf8(bad_utf8);
    assert!(result.is_err());
    // Since String::from_utf8 failed, it didn't consume the original
    // vector, and the error value hands ii back to us unharmed.
    println!("{}", format!("{:?}", result.unwrap_err().into_bytes()));
}
