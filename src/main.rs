
mod hand;
mod player;

fn main() {
    let test = hand::Card {
        suit: hand::Suit::House,
        value: hand::Value::Numeric(7)
    };
    println!("{:?}", test)
}
