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

    let haystack = "One fine dauy, in the middle of the night";
    if let Some(num) = haystack.find(',') {
        println!("{}", num);
    }
    if let Some(num) = haystack.find("night") {
        println!("{}", num);
    }
    if let Some(num) = haystack.find(char::is_whitespace) {
        println!("{}", num);
    }
    println!("{}", "## Elephants"
                   .trim_left_matches(|ch: char| ch == '#' || ch.is_whitespace()));
    let code = "\t    function nodele() {";
    println!("{}", code.trim_left_matches(&[' ', '\t'] as &[char]));

    let quip = "We also know there are known unknowns";
    if let Some(i) = quip.find("ya know") {
        println!("{}", i);
    } else {
        println!("\"ya know\" is None");
    }

    println!("{}", 
             "The only thing we have to fear is fear itself"
             .replace("fear", "spin"));
    println!("{}",
            "`Borrow` and `BorrowMut`"
            .replace(|ch:char| !ch.is_alphanumeric(), ""));

    let poem = "This is just  to say\n\
                I have eaten\n\
                the plums\n\
                again\n";
    for e in &poem.split_whitespace().collect::<Vec<_>>() {
        println!("{}", e);
    }
}

