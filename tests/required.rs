use constructor_lite::ConstructorLite;

#[derive(Debug, PartialEq, ConstructorLite)]
struct Movie {
    title: String,
    #[constructor(required)]
    year: Option<u16>,
}

#[test]
fn test_required() {
    assert_eq!(
        Movie::new("Star Wars".to_owned(), None),
        Movie {
            title: "Star Wars".to_owned(),
            year: None
        }
    )
}
