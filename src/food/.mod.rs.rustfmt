pub mod prelude {
    pub use super::Color;
    pub use super::Apple;
    pub use super::Edible;
    pub use super::Crunchy;
}

/// Defines module in src/food/color.rs
mod color;

/// Expose `Color`
/// Make `Color` useable with `use crate::food::Color`
pub use color::Color;

/// Generate debugging code for `Apple`,
/// so you can do
/// ```
/// let my_apple = Apple {
///     color: Color::Red,
/// };
/// println!("{:?}", my_apple)`
/// ```
#[derive(Debug)]
pub struct Apple {
    color: Option<Color>,
}

impl Apple {
    // Create new apple instance without color (`Option::None`)
    pub fn new() -> Apple {
        Apple {
            color: None
        }
    }

    /// Create new apple instance _with_ color (`Option::Some(Apple)`)
    pub fn with_color(color: Color) -> Apple {
        Apple {
            color: Some(color)
        }
    }

    pub fn color(&self) -> String {
        match self.color {
            //! The `format!()` [macro] lets us create a `String` with a pattern,
            //! equal to how the `println!()` macro does it.
            //! [macro]: https://doc.rust-lang.org/book/ch19-06-macros.html
            Some(color) => format!("This apple is {:?}", color),
            None        => String::from("This apple has no color :("),
        }
    }
}

/// You can implpement a trait such as `Edible` for other types (usually for structs).
/// ```
/// struct MyStruct; // Struct without fields
/// impl Edible for MyStruct {
///     // Define required methods here
///     fn taste(&self) -> &str {
///         "MySturct's taste"
///     }
/// }
/// ```
pub trait Edible {
    fn is_edible(&self) -> bool {
        true
    }

    fn taste(&self) -> &str;
}

pub trait Crunchy: Edible {
    fn is_crunchy(&self) -> bool {
        true
    }
}

/// Implement `Edible` and `Crunchy` traits for `Apple`
impl Edible for Apple {
    fn taste(&self) -> &str {
        "Mmmh, crunchy..."
    }
}

impl Crunchy for Apple {}
