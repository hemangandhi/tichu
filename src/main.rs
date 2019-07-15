#[macro_use]
extern crate cached;
mod tichu;

fn main() {
    let t = tichu::game::Game::New();
    println!("{:?}", t)
}
