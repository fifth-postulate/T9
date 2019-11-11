extern crate clap;
extern crate t9;

use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use t9::{pad, tree::Tree};

fn main() -> io::Result<()> {
    let matches = App::new("t9")
        .version("0.1.0")
        .author("Daan van Berkel <daan.v.berkel.1980@gmail.com>")
        .about("reverse lookup of port numbers")
        .arg(
            Arg::with_name("dictionary")
                .short("d")
                .long("dict")
                .value_name("FILE")
                .help("the dictionary file to use as candidate words")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .help("the port number to look up")
                .required(true)
                .index(1),
        )
        .get_matches();

    let mut tree = Tree::empty();
    let dictionary = matches
        .value_of("dictionary")
        .unwrap_or("/usr/share/dict/american-english");
    let file = File::open(dictionary)?;
    let reader = BufReader::new(file);
    for word in reader.lines() {
        tree.add(word?);
    }

    let port = matches.value_of("port").unwrap(); // safe because port is required
    let digits = pad::digits_for(port);
    let words = tree.words_at(digits);
    for word in words {
        println!("{}", word)
    }
    Ok(())
}
