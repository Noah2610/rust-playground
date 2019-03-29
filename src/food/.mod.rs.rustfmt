pub mod prelude {
    pub use super::Color;
    pub use super::Apple;
    pub use super::Edible;
    pub use super::Crunchy;
}

mod color;

pub use color::Color;

#[derive(Debug)]
pub struct Apple {
    color: Option<Color>,
}

impl Apple {
    pub fn new() -> Apple {
        Apple {
            color: None
        }
    }

    pub fn with_color(color: Color) -> Apple {
        Apple {
            color: Some(color)
        }
    }

    pub fn color(&self) -> String {
        match self.color {
            Some(color) => format!("This apple is {:?}", color),
            None        => String::from("This apple has no color :("),
        }
    }
}

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

impl Edible for Apple {
    fn taste(&self) -> &str {
        "Mmmh, crunchy..."
    }
}

impl Crunchy for Apple {}
