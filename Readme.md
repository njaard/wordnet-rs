[![GitHub license](https://img.shields.io/badge/license-BSD-blue.svg)](https://raw.githubusercontent.com/njaard/wordnet-rs/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/wordnet.svg)](https://crates.io/crates/wordnet)
[![Documentation](https://docs.rs/wordnet/badge.svg)](https://docs.rs/wordnet)

	[dependencies]
	wordnet = "0.1"

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
* Source-breaking changes will follow semantic versioning.

# Example

	let senses = wn.senses("horse");
	senses[0]
		.pointers.iter()
		.filter(|p| p.relationship == wordnet::Relationship::Hypernym)
		.map(|p| p.read())
		.for_each( |e| println!("a horse is a {}", e.synonyms[0].word));

	Output: A horse is an equine

# Robustness
If the database is corrupt then the library may panic.
I'm not clear on if there's a cause to use `Result`.

It is possible that there are bugs that may cause the
library to enter an infinite loop while parsing the database.

# Compatibility
The library is known to work on Linux and Windows.

# See Also
* [wordnet_stemmer](https://crates.io/crates/wordnet_stemmer) crate.
