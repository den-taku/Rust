fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}

fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;
    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal)     => "at the origin",
        (_, Equal)         => "on the x axis",
        (Equal, _)         => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater)    => "in the seconed quadrant",
        _                  => "somewhere else"
    }
}

struct Account {
    name: String,
    language: String,
    sex: String,
    age: u8
}

fn main() {
    let int = 5;
    match int {
        0 => {},
        1 => println!("A rabbit is nothing around in the clover."),
        n => println!("There are {} rabbits hopping about in the mewdow", n),
    }
    println!("{}", type_of({}));
    match "hoge" {
        "hoge" => println!("hoge!"),
        _ => panic!("invisible power effected")
    }
    println!("{}", describe_point(0, 9));

    let mut account = Account{ 
        name: "Dentaku".to_string(), 
        language: "Japanese".to_string(),
        sex: "Denki-Nezumi".to_string(),
        age: 3
    };
    {
        let r_acc = match account {
            Account { ref name, ref mut language, .. } => {
                println!("name: {}", *(&name));
                *language += " & English";
                println!("language: {}", *language);
            }
        };
        println!("{}", type_of(r_acc));
    }
    struct Point3d {
        x: i32,
        y: i32,
        z: i32
    }
    let s = Point3d{x:0, y: 3, z:0 };
    match &s {
        &Point3d{x, y, z} => {
            println!("{}", type_of(x));
        }
    }
    struct StringPoint1d {
        xs: String
    }
    let ss = StringPoint1d { xs: "32".to_string() };
    match &ss {
        &StringPoint1d {ref xs} => {
            println!("{}", type_of(xs));
        }
    }
    struct ReferencePoint<'elt> {
        x_ref: &'elt i32
    }
    let refed: i32 = 90;
    let rp = ReferencePoint { x_ref: &refed };
    match rp {
        ReferencePoint{ x_ref } => {
            println!("{}", type_of(x_ref));
        }
    }
    enum Option<T> {
        None,
        Some(T)
    }
    let opt = Option::<&i32>::Some(&refed);   
    match opt {
        Option::<&i32>::Some(ref r) => {
            println!("&r is {}.", type_of(r)) ;
        }
        _ => {}
    }   
    let r = &refed;
    // r += 90;
    println!("{}", *r + 89);

    println!("Hello, world!");
}
