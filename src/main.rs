use std::env;

mod Avdan;
mod api;


fn main() {
    let args: Vec<String> = env::args().collect();

    let r= Avdan::runtime::Runtime::new();
    r.run_extension(args);

}