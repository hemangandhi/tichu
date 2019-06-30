#[macro_use] extern crate cached;
mod tichu;

fn main() {
    let test = tichu::hand::Card {
        suit: tichu::hand::Suit::House,
        value: tichu::hand::Value::Numeric(7)
    };
    println!("{:?}", test)
}
