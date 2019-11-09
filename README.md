# T9
Predictive text in Rust

## Motivation
I learned a neat trick from [ag_dubs][] to come up with ports for your application. They used it during various [RustBridge][rustbridge] events.

The trick is to take a word that is relevant for your application and looking at the keypad of a phone.

![Keypad of a phone used by T9](https://en.wikipedia.org/wiki/T9_%28predictive_text%29#/media/File:Telephone-keypad2.svg)

Lookup the corresponding number for each letter and that will be your port number.

[ag_dubs]: https://twitter.com/ag_dubs
[rustbridge]: https://rustbridge.com/
