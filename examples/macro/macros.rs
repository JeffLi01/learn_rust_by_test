pub use std::collections::HashMap;
pub use std::boxed::Box;
pub use std::string::ToString;



#[macro_export]
macro_rules! json {
    (null) => {
        $crate::Json::Null
    };
    ([ $( $element:tt ),* ]) => {
        $crate::Json::Array(vec![ $( json!($element) ),* ])
    };
    ({ $( $key:tt : $value:tt ),* }) => {
        {
            let mut fields = $crate::macros::Box::new(
                $crate::macros::HashMap::new());
            $( fields.insert($crate::ToString::to_string($key), json!($value)); )*
            $crate::Json::Object(fields)
        }
    };
    ($other:tt) => {
        $crate::Json::from($other) // handle Boolean, Number and String
    }
}


#[cfg(test)]
mod test {
    use crate::Json;
    #[test]
    fn json_null() {
        assert_eq!(json!(null), Json::Null);
    }

    #[test]
    fn json_array_with_json_element() {
        let macro_generated_value = json!(
            [
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
}
