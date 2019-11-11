extern crate t9;

use clap::{App, Arg};
use t9::pad;

fn main() {
    let matches = App::new("port")
        .version("0.1.0")
        .author("Daan van Berkel <daan.v.berkel.1980@gmail.com>")
        .about("create port numbers from words")
        .arg(
            Arg::with_name("word")
                .help("the word to turn into a port number")
                .required(true)
                .index(1),
        )
        .get_matches();

    let word = matches.value_of("word").unwrap(); // safe because the word is required
    let port = pad::digits_for(word);
    println!("{}", port)
}
