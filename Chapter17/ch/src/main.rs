use std::fmt::Write;

fn main() {
    println!("{}", '4'.is_numeric());
    println!("{}", '\n'.is_control());
    println!("{}", ' '.is_whitespace());
    if let Some(num) = 'a'.to_digit(11) {
        println!("{}", num);
    }
    if let Some(ch) = std::char::from_digit(11, 16) {
        println!("{}", ch);
    }
    println!("{}", '\u{307}');
    println!("{}", '\u{130}');
    if let Some(num) = '3'.to_digit(10) {
        println!("{}", num);
    }
    let ch = '\u{766}';
    println!("{}", ch);
    println!("{}", ch as u8);

    let spacey = "man hat tan";
    let spaceless: String =
        spacey.chars().filter(|c| !c.is_whitespace()).collect();
    println!("{} -> {}", spacey, spaceless);

    let mut letter = String::new();
    writeln!(letter, "Whose {} these are I thinl I know", "rutabagas").expect("err");
    writeln!(letter, "His house is in village though;").expect("err");
    letter.pop();
    println!("{}", letter);

    let mut choco = "chocolate".to_string();
    println!("{}", choco.drain(3..6).collect::<String>());
    println!("{}", choco);

}
