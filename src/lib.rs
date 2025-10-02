pub struct Foo;

macro_rules! implement {
    ($name:ident) => {
        impl $name {
            pub fn new() -> Self {
                Self {}
            }
        }
    };
}
implement!(Foo);

mod practice;

#[macro_export]
macro_rules! ex {
    () => {};
}
