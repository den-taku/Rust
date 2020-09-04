use ref_with_flag::RefWithFlag;

mod ref_with_flag {
    use std::marker::PhantomData;
    use std::mem::align_of;

    /// A `&T` and a `bool`, wrapped up in a single word.
    /// The type `T` must require at least tyo-byte alignment.
    /// 
    /// If you're the kind of programmer who's never met a pointer whose
    /// 2^0-bit you didn't want to steal, well, now you can do it safety!
    /// ("But it's not nearly as exciting this way...")
    pub struct RefWithFlag<'a, T: 'a> {
        ptr_and_bit: usize,
        behaves_like: PhantomData<&'a T> // occupies no space
    }

    impl<'a, T: 'a> RefWithFlag<'a, T> {
        pub fn new(ptr: &'a T, flag: bool) -> RefWithFlag<T> {
            assert!(align_of::<T>() % 2 == 0);
            RefWithFlag {
                ptr_and_bit: ptr as *const T as usize | flag as usize,
                behaves_like: PhantomData
            }
        }

        pub fn get_ref(&self) -> &'a T {
            unsafe {
                let ptr = (self.ptr_and_bit & !1) as *const T;
                &*ptr
            }
        }

        pub fn get_flag(&self) -> bool {
            self.ptr_and_bit & 1 != 0
        }
    }
}

fn main() {
    let vec = vec![10, 20, 30];
    let flagged = RefWithFlag::new(&vec, true);
    println!("{:?}", flagged.get_ref());
    println!("{:?}", flagged.get_flag());
}
