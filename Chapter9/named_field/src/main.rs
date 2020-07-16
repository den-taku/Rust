/// A rectangle of eight-bit grayscale pixels.
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize)
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap{ pixels, size }
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent
}

/// Two possible alternatives for what a `Broom` could be working on.
#[derive(Copy, Clone)]
enum BroomIntent { FetchWater, DumpWater }

// Recieve the input Broom by value, taking ownership.
fn chop(b: Broom) -> (Broom, Broom) {
    // Initialize `bloom1` mostly from `b`, changing only `height`. Since
    // `String` is not `Copy`, `bloom1` takes ownership of `b`'s name.
    let mut bloom1 = Broom { height: b.height / 2, ..b };

    // Initialize `bloom2` mostly from `bloom1`. Since `String` is not
    // `Copy`, we must clone `name` explicitly.
    let mut bloom2 = Broom { name: bloom1.name.clone(), ..bloom1 };

        // Give each fragment a distinct name.
    bloom1.name.push_str(" I");
    bloom2.name.push_str(" II");
    
    (bloom1, bloom2)
}

struct Bounds(usize, usize);

fn main() {
    let tmp = ("hi".to_string(), 370i32);
    let (head, body) = ("Hey".to_string(), "PPAP".to_string());
    println!("{}", tmp.0);
    println!("{}", body);
 
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height)
    };
    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater
    };

    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.health, 100);
    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.health, 100);

    let image_bounds = Bounds(1024,768);
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);
    for i in 0..2 {
        println!("{}", image_bounds.0);
    }

    println!("Hello, world!");
}
