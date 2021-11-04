pub use std::string::ToString;
pub use std::collections::HashMap;

#[macro_use] mod macros;

#[derive(Clone, PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
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

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64);


fn main() {
    println!("Hello, world!");
    let width = 4.0;
    let desc = json!({
        "width": width,
        "height": (width * 9.0 / 4.0)
    });

    println!("{:#?}", desc);
}
