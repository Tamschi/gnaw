# gnaw

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/gnaw)
[![Crates.io](https://img.shields.io/crates/v/gnaw)](https://crates.io/crates/gnaw)
[![Docs.rs](https://docs.rs/gnaw/badge.svg)](https://docs.rs/crates/gnaw)

![Rust 1.40.0](https://img.shields.io/static/v1?logo=Rust&label=&message=1.40.0&color=grey)
[![Build Status](https://travis-ci.com/Tamschi/gnaw.svg?branch=develop)](https://travis-ci.com/Tamschi/gnaw/branches)
![Crates.io - License](https://img.shields.io/crates/l/gnaw/0.0.2)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/gnaw)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/gnaw)](https://github.com/Tamschi/gnaw/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/gnaw)](https://github.com/Tamschi/gnaw/pulls)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/gnaw.svg)](https://web.crev.dev/rust-reviews/crate/gnaw/)

With this crate, you can conveniently chip pieces off a [slice] or [`str`] to use elsewhere.

[slice]: https://doc.rust-lang.org/stable/std/primitive.slice.html
[`str`]: https://doc.rust-lang.org/stable/std/primitive.str.html

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add gnaw
```

## Example

```rust
use gnaw::{Drain as _, Pop as _, Unshift as _};

let mut text = "abcdefg";

assert_eq!(text.unshift(), Some('a'));
assert_eq!(text.pop(), Some('g'));

let mut drain = text.drain();
assert_eq!(drain.next(), Some('b'));
assert_eq!(drain.next(), Some('c'));
assert_eq!(drain.next_back(), Some('f'));
assert_eq!(drain.next_back(), Some('e'));
drop(drain);

//TODO: Why isn't the mutable borrow released here?
//assert_eq!(text, "d");
```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`gnaw` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

* The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
* The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).
