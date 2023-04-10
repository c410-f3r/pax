use pax_core::unsafe_unwrap;

#[derive(Debug, PartialEq)]
#[repr(u32)]
enum Fruit {
    Apple(String),
    Banana(String),
}

#[test]
fn test_unwrap_apple() {
    let fruit = Fruit::Apple("green".to_string());
    let expected_color = "green".to_string();
    let unwrapped_color: String =
        unsafe_unwrap!(fruit, Fruit, String);
    assert_eq!(unwrapped_color, expected_color);
}

#[test]
#[should_panic(expected = "The size_of target_type must be less than the size_of enum_type.")]
fn test_unwrap_invalid_size() {
    let fruit = Fruit::Apple("red".to_string());
    let _unwrapped_fruit: Fruit = unsafe_unwrap!(fruit, Fruit, Fruit);
}