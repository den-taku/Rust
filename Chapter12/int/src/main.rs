use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T, // inclusive
    upper: T  // exclusive
}

impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        if self == other { Some(Ordering::Equal) }
        else if self.lower >= other.upper { Some(Ordering::Greater) }
        else if self.upper <= other.lower { Some(Ordering::Less) }
        else { None }
    }
}

struct Image<P> {
    width: usize,
    pixels: Vec<P>
}

impl<P: Default + Copy> Image<P> {
    /// Create a new image of the given size.
    fn new(width: usize, hight: usize) -> Image<P> {
        Image {
            width,
            pixels: vec![P::default(); width * hight]
        }
    }
}   

impl<P> std::ops::Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, row:usize) -> &[P] {
        let start = row * self.width;
        &self.pixels[start .. start + self.width]
    }
}

impl<P> std::ops::IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut [P] {
        let start = row * self.width;
        &mut self.pixels[start .. start + self.width]
    }
}

fn main() {
    assert!(Interval { lower: 10, upper: 20 } <  Interval { lower: 20, upper: 40 });
    assert!(Interval { lower:  7, upper:  8 } >= Interval { lower:  0, upper:  1 });
    assert!(Interval { lower:  7, upper:  8 } <= Interval { lower:  7, upper:  8 });
    let left  = Interval { lower: 10, upper: 30 };
    let right = Interval { lower: 20, upper: 40 };
    assert!(!(left < right));
    assert!(!(left >= right));

    let mut m = HashMap::new();
    m.insert("十", 10);
    m.insert("百", 100);
    m.insert("千", 1000);
    m.insert("万", 1_0000);
    m.insert("億", 1_0000_0000);
    assert_eq!(m["十"], 10);
    assert_eq!(m["千"], 1000);
    
    use std::ops::Index;
    assert_eq!(*m.index("億"), 1_0000_0000);

    let mut desserts = vec!["Howalon".to_string(),
                            "Soan papdi".to_string()];
    desserts[0].push_str(" (fictional)");
    desserts[1].push_str(" (real)");
}

