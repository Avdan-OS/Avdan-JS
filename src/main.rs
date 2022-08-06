use std::env;

use Avdan::Runtime;

mod Avdan;
mod core;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut r = Runtime::new();
    r.run_extension(args).join();
}
