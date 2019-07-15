#[macro_use]
extern crate cached;
mod tichu;

fn main() {
    let t = tichu::player::Game::New();
    println!("{:?}", t)
}
