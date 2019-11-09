extern crate t9;

use std::env;
use t9::pad;

fn main() {
    let args: Vec<String> = env::args().collect();

    let port_name = &args[1];
    let port = pad::numbers_for(&port_name);
    println!("{}", port)
}
