use std::borrow::Cow;

fn get_name() -> Cow<'static, str> {
    std::env::var("USER")
        .map(|v| v.into())
        .unwrap_or("whoever you are".into())

    // std::env::var("USER")
    //     .map(|v| Cow::Owned(v))
    //     .unwrap_or(Cow::Borrowed("whoever you are"))
}

fn main() {
    println!("Greetings, {}!", get_name());
    let title = "Esq.";
    let mut name = get_name();
    // name.to_mut().push_str(", ");
    name += ", ";
    // name.to_mut().push_str(title);
    name += title;
    // or write!(name.to_mut(), ", {}", title).unwrap();
    println!("Greetings, {}!", name);
}
