macro_rules! KobeToKyoto {
    ($kobe:expr) => {
        match $kobe {
            "Kobe" => "Kyoto",
            _ => ""
        }
    }
}
        

fn main() {
    println!("Hello, {}!", KobeToKyoto!("Kobe"));
}
