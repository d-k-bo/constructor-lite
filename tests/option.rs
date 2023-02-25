use constructor_lite::ConstructorLite;

#[derive(Debug, PartialEq, ConstructorLite)]
struct Movie {
    title: String,
    year: Option<u16>,
    genres: core::option::Option<Vec<String>>,
    director: ::core::option::Option<String>,
    composer: std::option::Option<String>,
    cast: ::std::option::Option<Vec<String>>,
}

#[test]
fn test_option() {
    assert_eq!(
        Movie::new("Star Wars".to_owned()),
        Movie {
            title: "Star Wars".to_owned(),
            year: None,
            genres: None,
            director: None,
            composer: None,
            cast: None
        }
    )
}
