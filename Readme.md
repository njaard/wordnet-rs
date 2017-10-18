# Introduction

This is a parser for [Wordnet](https://wordnet.princeton.edu/)
dictionaries, written in 100% Rust.

# Completeness

The library poorly tested and incomplete.

Here are the unimplemented features:

* [lexicographical file numbers](https://wordnet.princeton.edu/man/lexnames.5WN.html)
* handling of word numbers in the synset
* verb exceptions
* verb sentences
* many more things
* There may be some `Send` and `Sync` traits that could be applied.

# Robustness
If the database is corrupt then the library may panic.
I'm not clear on if there's a cause to use `Result`.

It is possible that there are bugs that may cause the
library to enter an infinite loop while parsing the database.

# Compatibility
The library is known to work on Linux and Windows.

# See Also
* [wordnet_stemmer](https://crates.io/crates/wordnet_stemmer) crate.
