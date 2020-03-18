/// macros.rs
/// This file contains meta-code that is used mainly to avoid writing repeated code, for the math library.

/// implement_point: implements a Point struct in code.
macro_rules! implement_point {
    ($name: ident, {
        $elements: item
    },
    {
        $($values: ident),*
    }) => {
        pub struct $name $elements
        impl Default for $name {
            fn default() -> Self {
                Self $values
            }
        }
    };
}
