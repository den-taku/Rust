macro_rules! _assert_eq2 {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_var, right_var) => {
                if !(*left_var == *right_var) {
                    panic!("assertion failed: `(left == right)` \
                            (left: `{:?}`, right: `{:?}`)",
                            left_var, right_var)
                }
            }
        }
    });
}

// macro_rules! STAY {
//     (tmp: i32) => ( tmp );
// }

/*hoge*/

// fn hoge(expr: expr) -> () {
//     expr;
// }

macro_rules! print_assert {
    ($elem:expr) => {
        println!("{}", stringify!($elem));
    };
}

macro_rules! vec {
    ($elem:expr ; $n:expr) => {
        ::std::vec::from_elem($elem, $n)
    };
    ( $( $x:expr ),* ) => {
        <[_]>::into_vec(Box::new([ $( $x ),* ]))
    };
    ( $( $x:expr ),+ ,) => { // if trailing comma is present,
        vec![ $( $x ), * ]   // retry without it
    };
}

fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}

fn foo() {
    ()
}

fn foofoo() {
    if true {
        ;
    }
    ()
}

fn main() {
    // Repeat a value N times
    let _buffer = vec![0_u8; 1000];

    // A list of values, separated by commas
    let _numbers = vec!["udon", "ramen", "soba"];

    // let _challenge = vec!["hoge", "huge"; 500];

    // assert_eq2!(1,2);
    // assert_eq!(1,2);
    // println!("Hello, world!");
    // println!("{}", STAY!(89_i32));

    print_assert!({let hoge = 45.6;});

    // let foo = "foo";
    let bar = "bar".to_string();
    // println!("{}", concat!("hoge", "huga".to_string(), foo, bar));

    let version = env!("CARGO_PKG_VERSION");
    println!("{}", version);

    println!("{}", include!("huga.rs"));
    include!("hoge.rs");

    println!("{}", type_of({let a = 0;}));

    if true {
        ;
    };

    foo();

    if true {
        89_i32
    } else {
        34_i32
    };

    if true {
        ;
    }

    45_i32;
    
}
