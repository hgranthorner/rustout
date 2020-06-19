mod utils;
mod block;
mod consts;
mod models;
mod paddle;
mod game;

#[allow(unused_imports)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

fn main() {
    println!("Hello, world!");
    game::run();
}
