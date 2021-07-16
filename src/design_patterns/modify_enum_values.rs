#[test]
fn test_single () {
    use std::mem;

    #[derive(Debug, PartialEq)]
    enum MyEnum {
        A { name: String, x: u8 },
        B { name: String }
    }
    
    fn a_to_b(e: &mut MyEnum) {
        if let MyEnum::A { name, x: 0 } = e {
            // this takes out our `name` and put in an empty String instead
            // (note that empty strings don't allocate).
            // Then, construct the new enum variant (which will
            // be assigned to `*e`).
            *e = MyEnum::B { name: mem::take(name) };
        }
    }

    let mut a = MyEnum::A{ name: "A".to_string(), x: 0 };
    let b = MyEnum::B{ name: "A".to_string() };
    a_to_b(&mut a);
    assert_eq!(a, b);
}

#[test]
fn test_more_variants () {
    use std::mem;

    #[derive(Debug, PartialEq)]
    enum MultiVariateEnum {
        A { name: String },
        B { name: String },
        C,
        D
    }
    
    fn swizzle(e: &mut MultiVariateEnum) {
        use MultiVariateEnum::*;
        *e = match e {
            // Ownership rules do not allow taking `name` by value, but we cannot
            // take the value out of a mutable reference, unless we replace it:
            A { name } => B { name: mem::take(name) },
            B { name } => A { name: mem::take(name) },
            C => D,
            D => C
        };
    }
    let mut a = MultiVariateEnum::A{ name: "A".to_string() };
    let b = MultiVariateEnum::B{ name: "A".to_string() };
    swizzle(&mut a);
    assert_eq!(a, b);
}
