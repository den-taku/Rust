#[derive(Debug)]
struct Same {
    field1: i32,
    field2: i32,
    field3: i32,
    field4: i32,
    field5: i32
}

impl Same {
    fn new(def: i32) -> Same {
        Same {
            field1: def,
            field2: def,
            field3: def,
            field4: def,
            field5: def
        }
    }
}

#[derive(Debug)]
struct U32(u32);

#[derive(Debug)]
struct NoNameStructure;

fn main() {
    let same1 = Same::new(1);
    let same2 = Same::new(2);
    let same3 = Same { field1: 3, field4: 5, .. same1 };
    let same4 = Same { field4: 5, field3: 3, .. same2 };
    println!("{:#?}", same1);
    println!("{:#?}", same2);
    println!("{:#?}", same3);
    println!("{:#?}", same4);
    println!("{:#?}", U32(78));
    println!("{:#?}", NoNameStructure);
}
