// use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use std::collections::HashMap;
use std::io;
use serde::Serialize;
use serde::Deserialize;
use serde_json::Serializer;

#[derive(Serialize, Deserialize)]
struct Player {
    location: String,
    items: Vec<String>,
    health: u32
}

fn main() {
    // let n = reader.read_u32::<LittleEndian>().expect("err");
    // let mut gzip_reader = file.gz_decode().expect("err");

    type RoomId = String;
    type RoomExits = Vec<(char, RoomId)>;
    type RoomMap = HashMap<RoomId, RoomExits>;

    // Create a simple map.
    let mut map = RoomMap::new();
    map.insert("Cobble Crawl".to_string(),
               vec![('W', "Debris Room".to_string())]);
    map.insert("Deris Room".to_string(),
               vec![('E', "Cobble Crawl".to_string()),
                    ('W', "Sloping Canyon".to_string())]);
    
    let mut serializer = Serializer::new(io::stdout());
    map.serialize(&mut serializer).expect("serialize error");
    println!("{}", ' ');
    let player = Player { location: "Deris Room".to_string(), 
                          items: Vec::<String>::new(),
                          health: 32};
    player.serialize(&mut serializer).expect("err");
    println!(" ");
}
