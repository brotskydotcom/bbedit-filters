# bbedit-filters

This crate contains a number of _text filters_ for use with [BBEdit](https://barebones.com/bbedit). BBEdit text filters are executables that read the standard input and write the standard output.  BBEdit invokes the filter with the selected section of the buffer as input, and then replaces that buffer section with the filter's output.

## Base64 Filters

There are two filters provided for Base64: one that decodes the selection from Base64URL format (with no padding), and one that encodes the selection into Base64URL format (with no padding).

## JSON filters

There is one filter provided for JSON: it pretty-prints the selection.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
