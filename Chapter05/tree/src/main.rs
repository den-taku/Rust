fn main() {
    let mut x = 10;
    let r1 = &x;
    let r2 = &x;
    // x += 10;

    // let m = &mut x;

    let mut y = 20;
    let m1 = &mut y;
    // let m2 = &mut y;
    // let mut z = y;

    let mut w = (107, 109);
    let r = &w;
    let r0 = &r.0;
    // let m1 = &mut r.1;

    let  mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0;
    *m0 = 137;

    let r1 = &m.1;
    // v.1;

    // struct File {
    //     descriptor: i32
    // }

    // fn new_file(d: i32) -> File {
    //     File { descriptor: d }
    // }

    // fn clone_from(this: &mut File, rhs: &File) {
    //     close(this.descriptor);
    //     this.descriptor = dup(rhs.descriptor);
    // }
    
}
