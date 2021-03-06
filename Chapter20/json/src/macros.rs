// macros.rs
pub use std::collections::HashMap;
pub use std::boxed::Box;
pub use std::string::ToString;

#[macro_export]
macro_rules! json {
    (null) => {
        $crate::macros::Json::Null
    };
    ([ $( $element:tt ),* ]) => {
        $crate::macros::Json::Array(vec![ $( json!($element) ),* ])
    };
    ({ $($key:tt : $value:tt),* }) => {
        {
            let mut fields = $crate::macros::Box::new(
                $crate::macros::HashMap::new());
            $( fields.insert($crate::macros::ToString::to_string($key), json!($value)); )*
            $crate::macros::Json::Object(fields)
        }
    };
    ($other:tt) => {
        $crate::macros::Json::from($other) // Handle Boolean/number/string
    };
}

macro_rules! impl_from_num_for_json {
    ( $( $t:ident )* ) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Json {
                    Json::Number(n as f64)
                }
            }
        )*
    };
}

#[test]
fn json_null() {
    assert_eq!(json!(null), Json::Null);
}

#[test]
fn json_array_with_json_element() {
    let macro_generated_value = json!(
        [
            // valid Json that doesn't match `$element:expr`
            {
                "pitch": 440.0
            }
        ]
    );
    let hand_coded_value =
        Json::Array(vec![
            Json::Object(Box::new(vec![
                ("pitch".to_string(), Json::Number(440.0))
            ].into_iter().collect()))
        ]);
    assert_eq!(macro_generated_value, hand_coded_value);
}

#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>)
}

impl From<bool> for Json {
    fn from(b:bool) -> Json {
        Json::Boolean(b)
    }
}

// impl From<i32> for Json {
//     fn from(i: i32) -> Json {
//         Json::Number(i as f64)
//     }
// }

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}

fn main() {
    impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64);

    let width = 4.0;
    let desc =
        json!({
            "width": width,
            "height": (width * 9.0 / 4.0)
        });
    println!("{:#?}", desc);

    let fields = "Fields, W.C.";
    let role = json!({
        "name": "Larson E. Whipsnade",
        "actor": fields
    });
    println!("{:#?}", role);
}
