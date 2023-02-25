use constructor_lite::ConstructorLite;

#[derive(Debug, PartialEq, ConstructorLite)]
#[constructor(name = "from_name")]
struct Movie {
    title: String,
    year: Option<u16>,
}

#[test]
fn test_custom_constructor_name() {
    assert_eq!(
        Movie::from_name("Star Wars".to_owned()),
        Movie {
            title: "Star Wars".to_owned(),
            year: None
        }
    )
}
