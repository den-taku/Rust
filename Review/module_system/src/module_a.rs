pub mod module_a {
    pub type Int = i32;
    pub type Float = f32;

    #[derive(Debug)]
    pub struct Vector3d {
        x: Float,
        y: Float,
        z: Float
    }

    impl Vector3d {
        pub fn new(x: Float, y: Float, z: Float) -> Vector3d {
            Vector3d { x, y, z }
        }
    }
}
