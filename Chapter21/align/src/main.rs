fn main() {
    println!("std::mem::size_of::<i64>  : {}", std::mem::size_of::<i64>());
    println!("std::mem::align_of::<i64> : {}", std::mem::align_of::<i64>());

    // Fat pointers to slices carry their referent's length.
    let slice: &[i32] = &[1, 3, 9, 27, 81];
    println!("std::mem::size_of_val(slice) : {}", std::mem::size_of_val(slice));

    let text: &str = "alligator";
    println!("std::mem::size_of_val(text) : {}", std::mem::size_of_val(text));

    use std::fmt::Display;
    let unremarkable: & dyn Display = &193_u8;
    let remarkable: & dyn Display = &0.0072973525664;

    // These return the size/alignment of the vale the trait object points to,
    // not those of the trait object itself.
    // This information comes from the vtable the trait object refers to.

    println!("unremarkable : {}", unremarkable);
    println!("std::mem::size_of_val(unremarkable) : {}", std::mem::size_of_val(unremarkable));
    println!("remarkable : {}", remarkable);
    println!("std::mem::align_of_val(remarkable) : {}", std::mem::align_of_val(remarkable));
}
