# T9
Predictive text in Rust

## Motivation
I learned a neat trick from [ag_dubs][] to come up with ports for your application. They used it during various [RustBridge][rustbridge] events.

The trick is to take a word that is relevant for your application and looking at the keypad of a phone.

![Keypad of a phone used by T9](http://fifth-postulate.nl/T9/image/Telephone-keypad2.svg)

> By Sakurambo - Created using Adobe Illustrator CS2, Public Domain, [https://commons.wikimedia.org/w/index.php?curid=2048341](https://commons.wikimedia.org/w/index.php?curid=2048341)

Lookup the corresponding number for each letter and that will be your port number.

### Example
Let's take the word `rust`. Looking at the a keypad we find the following numbers

| Letter | Number |
|--------|--------|
| r      | 7      |
| u      | 8      |
| s      | 7      |
| t      | 8      |

To the port number for your application would be `7878`.

## Hard work
Figuring out the correct keypad is hard work. Luckily that weher the `t9` crate comes in. The following code could be used to figure the corresponding port for rust as well.

```rust
extern crate t9;

use std::env;
use t9::pad;

fn main() {
    let args: Vec<String> = env::args().collect();

    let port_name = &args[1];
    let port = pad::digits_for(port_name);
    println!("{}", port)
}
```

Calling it with `cargo run --examples port -- rust` returns `7878` as expected. [`port.rs`][port] can be found in the examples directory.

## Reverse lookup
`t9` also allows you to do a reverse lookup. I.e., what word was used for a given port number?

```rust
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
```

Run with `cargo run --release --example t9 -- 7878` returns the `rust` as expected. An more elaborate [example][t9], allowing you to specify the dictionary file, can be found in the examples directory.

[ag_dubs]: https://twitter.com/ag_dubs
[rustbridge]: https://rustbridge.com/
[port]: https://github.com/fifth-postulate/T9/blob/master/examples/port.rs 
[t9]: https://github.com/fifth-postulate/T9/blob/master/examples/t9.rs 

