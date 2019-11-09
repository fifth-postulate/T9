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

[ag_dubs]: https://twitter.com/ag_dubs
[rustbridge]: https://rustbridge.com/
