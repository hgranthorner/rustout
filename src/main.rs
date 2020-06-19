mod consts;
mod game_loop;
mod models;
mod utils;

#[allow(unused_imports)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

fn main() {
    println!("Hello, world!");
    game_loop::run();
}
