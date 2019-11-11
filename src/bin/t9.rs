extern crate t9;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use t9::{pad, tree::Tree};

fn main() -> io::Result<()> {
    let mut tree = Tree::empty();
    let file = File::open("/usr/share/dict/american-english")?;
    let reader = BufReader::new(file);
    for word in reader.lines() {
        tree.add(word?);
    }

    let args: Vec<String> = env::args().collect();

    let port = &args[1];
    let digits = pad::digits_for(port);
    let words = tree.words_at(digits);
    for word in words {
        println!("{}", word)
    }
    Ok(())
}
