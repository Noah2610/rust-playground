mod food;

use food::prelude::*;

fn main() {
    let apple = Apple::with_color(Color::Red);
    let apple_colorless = Apple::new();

    println!("{:?}", apple.color());
    println!("{:?}", apple_colorless.color());
}
