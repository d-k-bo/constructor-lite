use constructor_lite::ConstructorLite;

#[derive(Debug, PartialEq, ConstructorLite)]
struct Movie {
    #[constructor(default)]
    title: String,
    year: Option<u16>,
}

#[test]
fn test_default() {
    assert_eq!(
        Movie::new(),
        Movie {
            title: "".to_owned(),
            year: None
        }
    )
}
