extern crate wordnet;

fn main()
{
	let wn = wordnet::Database::open(&::std::path::Path::new("/usr/share/wordnet")).unwrap();

	let senses = wn.senses("horse");
	senses[0]
		.pointers.iter()
		.filter(|p| p.relationship == wordnet::Relationship::Hypernym)
		.map(|p| p.read())
		.for_each( |e| println!("a horse is a {}", e.synonyms[0].word));
}

