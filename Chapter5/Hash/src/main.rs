use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;    

fn show(table: & Table) {
    for ( artist, works ) in table {
        println!("Works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works ) in table {
        works.sort();
    }
} 


fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                 vec!["many madrigals".to_string(),
                      "Tenebrae Responsorai".to_string()]);
    table.insert("Caravaggio".to_string(),
                 vec!["The Musicians".to_string(),
                      "The Calling of St. Mattew".to_string()]);
    table.insert("Cellini".to_string(),
                 vec!["Perseus  with the head of Medusa".to_string(),
                     "a salt celler".to_string()]);

    sort_works(&mut table);

    show(&table);   
    assert_eq!(table["Gesualdo"][0], "many madrigals");

    let x = 10;
    let r = &x;
    assert!(*r == 10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    // assert!(*m == 32);

    struct Anime { name: &'static str, bechdel_pass: bool };
    let aria = Anime { name: "Aria: The Animarion", bechdel_pass: true };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation"); 
    assert_eq!((*anime_ref).name, "Aria: The Animation");
}
