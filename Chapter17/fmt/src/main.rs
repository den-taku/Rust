fn main() {
    println!("{:.3}μs: relocated {} at {:#x} to {:#x}, {} bytes",
        0.84391, "object",
        140737488346304_usize, 6299664_usize, 64);
    println!("{{a, c}} ⊂ {{a, b, c}}");
    println!("{:*<4}", "th\u{e9}");
    println!("{:*<4}", "th\u{301}");
}
