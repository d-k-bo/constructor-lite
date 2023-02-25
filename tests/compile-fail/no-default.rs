use constructor_lite::ConstructorLite;

struct DoesNotImplementDefault();

#[derive(ConstructorLite)]
struct Foo {
    #[constructor(default)]
    bar: DoesNotImplementDefault,
    //~^ ERROR mismatched types
}

fn main() {}
