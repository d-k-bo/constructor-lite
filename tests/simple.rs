use constructor_lite::ConstructorLite;

#[derive(Debug, PartialEq, ConstructorLite)]
struct Movie {
    title: String,
    year: Option<u16>,
}

#[test]
fn test_simple() {
    assert_eq!(
        Movie::new("Star Wars".to_owned()),
        Movie {
            title: "Star Wars".to_owned(),
            year: None
        }
    )
}
