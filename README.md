# sentence-parser

This is a very basic parser meant to be an example of how to use `proptest` to write property-based tests in Rust.

This parser attempts to parse a sentence, with a very loose definition of "sentence." We define a "word" to be any sequence of one or more ASCII letters (case doesn't matter). Some of the words may also be enclosed in delimiters such as commas or parentheses. A "sentence" then consists of a sequence of words, some of which may be enclosed in delimiters, followed by valid punctuation (period, question mark, or exclamation point). In this framework `(a) b.` is a valid sentence.

The `pest` crate is used to write the parser because the grammar file lets you declaratively define the parsing rules. This is the entire definition of the parser:
```
word = { ASCII_ALPHA+ }
words = ${ word ~ (" " ~ word)* }
enclosed = ${
    "(" ~ words ~ ")" |
    ", " ~ words ~ ","
}
chunk = ${ words | enclosed }
punctuation = { "." | "!" | "?" }
sentence = ${ SOI ~ chunk ~ (" " ~ chunk)* ~ punctuation ~ EOI }
WHITESPACE = _{" "}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
