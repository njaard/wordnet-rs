mod file_platform;
use file_platform::ReadAtFile;

use std::cmp::Ordering;
use std::str::from_utf8;

fn read_line_at(f : &ReadAtFile, mut pos : u64)
	-> Vec<u8>
{
	let mut block = vec![];
	let mut writing_to = 0usize;
	block.resize(512, 0u8);

	loop
	{
		let len = f.read_at(&mut block[writing_to..], pos).unwrap();
		let truncate = block.iter()
			.skip(writing_to)
			.enumerate()
			.find(|x| *x.1 == b'\n')
			.map(|x| x.0);
		if let Some(t) = truncate
		{
			block.truncate(writing_to+t);
			return block;
		}
		writing_to += len;
		pos += len as u64;
		let newlen = block.len()*2;
		block.resize(newlen, 0u8);
	}
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum PartOfSpeech
{
	/// An substantive
	Noun,
	/// a word that describes a noun
	Adjective,
	/// a word that describes a noun
	AdjectiveSatellite,
	/// a word that describes an action
	Verb,
	/// a word that describes a verb
	Adverb,
}


/// noun, adjective, verb, adverb
impl PartOfSpeech
{
	/// Returns a short dictionary-like label:
	///
	/// * n
	/// * adj
	/// * v
	/// * adv
	pub fn short(&self) -> &'static str
	{
		match *self
		{
			PartOfSpeech::Noun => "n",
			PartOfSpeech::Adjective => "adj",
			PartOfSpeech::AdjectiveSatellite => "adj",
			PartOfSpeech::Verb => "v",
			PartOfSpeech::Adverb => "adv",
		}
	}
}

fn part_of_speech_code_to_part_of_speech(code : &[u8])
	-> PartOfSpeech
{
	match code
	{
		b"n" => PartOfSpeech::Noun,
		b"v" => PartOfSpeech::Verb,
		b"a" => PartOfSpeech::Adjective,
		b"s" => PartOfSpeech::AdjectiveSatellite,
		b"r" => PartOfSpeech::Adverb,
		_ => panic!("impossible part of speech '{}'", from_utf8(code).unwrap()),
	}
}

/// Relates one word to another semantically
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Relationship
{
	/// an opposite word
	Antonym,
	/// broader forms of this word (a *structure* is a hypernym of a *building*)
	Hypernym,
	/// broader forms of this word of which this word is a specific instance
	/// (*The Enlightenment* is a specific instance of a *historic period*)
	InstanceHypernym,
	/// more specific versions of this word (a *courthouse* is a hyponym of a *house*)
	Hyponym,
	/// this word is a member of (the *world* is a hyponym of the *solar system*)
	MemberHolonym,
	/// this word is made with (*tin* is a substance holonym of *cassiterite*)
	SubstanceHolonym,
	/// this word is a part of (*land* is a part holonym of the *world*)
	PartHolonym,
	/// reverse of MemberHolonym (an *air bag* is a member meronym of *car*)
	MemberMeronym,
	/// reverse of SubstanceHolonym (*cassiterite* is a substance meronym of *tin*)
	SubstanceMeronym,
	/// reverse of PartHolonym (a *car* is a part holonym of an *air bag*)
	PartMeronym,
	/// *scientific* is an attribute of *scientific knowledge*
	Attribute,
	/// the word is related to (the adjective *outward* is an related to *outwardness*)
	DerivationallyRelated,
	///
	DomainOfTopic,
	///
	MemberOfTopic,
	///
	DomainOfRegion,
	///
	MemberOfRegion,
	///
	DomainOfUsage,
	///
	MemberOfUsage,

	/// A verb requires an action to be completed first (to *eat* requires one to *chew*)
	Entailment,
	/// A verb causes another action (to *retire* causes one to *yield*)
	Cause,
	///
	AlsoSee,
	///
	VerbGroup,

	///
	SimilarTo,
	///
	VerbParticiple,

	///
	PertainymOrDerivedFromAdjective, // fixme
}

fn relationship_code_to_relationship(code : &[u8])
	-> Relationship
{
	match code
	{
		b"!"	=> Relationship::Antonym,
		b"@"	=> Relationship::Hypernym,
		b"@i" => Relationship::InstanceHypernym,
		b"~"	=> Relationship::Hyponym,
		b"~i" => Relationship::InstanceHypernym,
		b"#m" => Relationship::MemberHolonym,
		b"#s" => Relationship::SubstanceHolonym,
		b"#p" => Relationship::PartHolonym,
		b"%m" => Relationship::MemberMeronym,
		b"%s" => Relationship::SubstanceMeronym,
		b"%p" => Relationship::PartMeronym,
		b"="	=> Relationship::Attribute,
		b"+"	=> Relationship::DerivationallyRelated,
		b";c" => Relationship::DomainOfTopic,
		b"-c" => Relationship::MemberOfTopic,
		b";r" => Relationship::DomainOfRegion,
		b"-r" => Relationship::MemberOfRegion,
		b";u" => Relationship::DomainOfUsage,
		b"-u" => Relationship::MemberOfUsage,
		b"*"	=> Relationship::Entailment,
		b">"	=> Relationship::Cause,
		b"^"	=> Relationship::AlsoSee,
		b"$"	=> Relationship::VerbGroup,
		b"&"	=> Relationship::SimilarTo,
		b"<"	=> Relationship::VerbParticiple,
		b"\\" => Relationship::PertainymOrDerivedFromAdjective,
		_ => panic!("illegal relationship code")
	}
}


/// Senses are different definitions or etymologies for a word.
///
/// The senses are also arranged by part of speech. For example,
/// "bank" has many senses, one is a verb that means "to count on something",
/// another is a noun that refers to the financial institution.
///
/// A list of these can be accessed by `senses()`
#[derive(Debug)]
pub struct Sense<'db>
{
	/// The part of speech that this sense has
	pub part_of_speech : PartOfSpeech,
	/// A short dictionary-like text written in prose that describes the word
	pub gloss : String,
	/// Ways to write this sense, one of which is
	/// probably the word you passed to `Database::senses()`
	pub synonyms : Vec<SenseRef>,
	/// Words that are somehow related to this sense.
	pub pointers : Vec<PointerRef<'db>>,
}

/// Connects a Sense to words that relationship
///
/// A PointerRef has not been loaded from the database yet. You
/// can call `read()` to do that.
#[derive(Debug)]
pub struct PointerRef<'db>
{
	db : &'db Database,
	/// The relationship this pointer has
	/// from the original word to to the sense
	/// you can read with `read()`
	pub relationship : Relationship,
	/// The part of the speech that this new sense has.
	pub part_of_speech : PartOfSpeech,
	offset : u64,
}

///
impl<'db> PointerRef<'db>
{
	/// Read this pointer from the database files.
	/// This might lead to a Sense that you already have
	/// seen so be careful to not recurse indefinitely.
	///
	/// If you only use look at once `relationship`, then everything
	/// should be ok
	pub fn read(&self) -> Sense<'db>
	{
		self.db
			.dbfile_for_part_of_speech(&self.part_of_speech)
			.read_sense(self.db, self.offset)
	}
}

#[derive(Debug)]
/// refers to the actual text of a word
pub struct SenseRef
{
	/// the word
	pub word : String,
	lex_id : u32,
}

impl SenseRef
{
}

#[derive(Debug)]
struct DBFile
{
	name : String,
	index : ReadAtFile,
	index_size : u64,
	data : ReadAtFile,
	part_of_speech : PartOfSpeech,
}

impl DBFile
{
	fn new(
		part_of_speech : PartOfSpeech,
		index : &std::path::Path,
		data : &std::path::Path
	)
		-> std::io::Result<DBFile>
	{
		let mut index_f = std::fs::File::open(index)?;
		let data_f = std::fs::File::open(data)?;

		let index_size = std::io::Seek::seek(&mut index_f, std::io::SeekFrom::End(0))?;

		Ok(DBFile
		{
			name: index.to_str().unwrap().to_string(),
			index: ReadAtFile::new(index_f),
			index_size: index_size,
			data: ReadAtFile::new(data_f),
			part_of_speech : part_of_speech,
		})
	}

	fn is_found_here(
		&self,
		pos : u64,
		data : &[u8],
		remaining_word : &[u8]
	) -> Ordering
	{
		for x in 0..data.len()
		{
			if x == remaining_word.len() && data[x] == b' '
			{
				return Ordering::Equal;
			}
			else if x >= remaining_word.len() || data[x] > remaining_word[x]
			{
				return Ordering::Less;
			}
			else if data[x] < remaining_word[x]
			{
				return Ordering::Greater;
			}
		}
		let block = &mut [0u8;32];
		let bytes = self.index.read_at(block, pos+data.len() as u64).unwrap();
		return self.is_found_here(
			pos + data.len() as u64,
			&block[0..bytes],
			&remaining_word[data.len()..]
		);
	}

	fn find_position(&self, word : &[u8])
		-> Option<u64>
	{
		let block = &mut [0u8;32];

		let mut end = self.index_size;
		let mut begin = 0u64;
		let mut pos = end/2;

		while end-begin > (word.len()+10) as u64
		{
			if end-pos < 32
			{
				pos = begin;
			}

			let bytes = self.index.read_at(block, pos).unwrap();

			let block = &block[ 0 .. bytes ];

			if pos == begin
			{
				begin += bytes as u64;
			}


			if let Some(newline_offset)
				= block.iter().enumerate().find(|a| *a.1 == b'\n').map(|x| x.0)
			{
				let newline = &block[newline_offset+1..];
				let current_line_starts_at = pos + newline_offset as u64 + 1;
				let rel = self.is_found_here(current_line_starts_at, newline, word);
				match rel
				{
					Ordering::Equal => return Some(current_line_starts_at),
					Ordering::Less =>
					{
						end = current_line_starts_at;
					},
					Ordering::Greater =>
					{
						begin = current_line_starts_at+word.len() as u64;
					}
				}

				if begin >= end { break; }

				let newpos = (end-begin)/2 + begin;
				if newpos == pos
				{
					break;
				}
				else
				{
					pos = newpos;
				}
			}
			else if (pos + bytes as u64) < end
			{
				pos += bytes as u64;
			}
			else
			{
				pos -= std::cmp::min(64, pos);
			}
		}

		None
	}

	fn read_sense<'db>(
		&self,
		database : &'db Database,
		offset : u64
	) -> Sense<'db>
	{
		let line = read_line_at(&self.data, offset);

		let sections : Vec<_> = line.split(|x| *x == b' ').collect();

		let part_of_speech = part_of_speech_code_to_part_of_speech(sections[2]);

		let mut index = 3;

		let synonyms_cnt =
			usize::from_str_radix(from_utf8(sections[index]).unwrap(), 16).unwrap();
		index += 1;

		let mut synonyms = vec!();
		synonyms.reserve(synonyms_cnt);

		for _sn in 0..synonyms_cnt
		{
			synonyms.push(
				SenseRef
				{
					word : from_utf8(sections[index])
						.unwrap()
						.chars()
						.map(|x| if x=='_' { ' ' } else { x })
						.collect(),
					lex_id : u32::from_str_radix(from_utf8(sections[index+1]).unwrap(), 16).unwrap(),
				}
			);
			index += 2;
		}
		let pointer_count =
			u32::from_str_radix( from_utf8(sections[index]).unwrap(), 10).unwrap();
		index+=1;

		let mut pointers = vec!();
		pointers.reserve(pointer_count as usize);

		for _pointern in 0..pointer_count
		{
			let rel = relationship_code_to_relationship(sections[index]);
			let offset = u64::from_str_radix(
				from_utf8(sections[index+1]).unwrap(), 10
			).unwrap();
			let part_of_speech = part_of_speech_code_to_part_of_speech(sections[index+2]);
			let _offset = u64::from_str_radix(
				from_utf8(sections[index+3]).unwrap(), 16
			).unwrap();

			index += 4;
			pointers.push(
				PointerRef
				{
					db: database,
					relationship : rel,
					part_of_speech: part_of_speech,
					offset : offset,
				}
			);
		}

		if sections[2] == b"v"
		{
			let frame_count =
				usize::from_str_radix(from_utf8(sections[index]).unwrap(), 10).unwrap();
			index += frame_count + 1;
		}

		let _ = index;

		let gloss =
		{
			let line_utf = from_utf8(&line).unwrap();
			let gloss = &line_utf[line_utf.find('|').unwrap()+2..];
			gloss
		};

		Sense
		{
			part_of_speech: part_of_speech,
			gloss: gloss.to_string(),
			synonyms: synonyms,
			pointers: pointers,
		}
	}

	/// Searches for a word and returns a list of all its senses.
	fn senses<'db>(&self, database : &'db Database, word : &[u8])
		-> Option<Vec<Sense<'db>>>
	{
		let offset = self.find_position(word);
		if offset.is_none() { return None; }

		let offset = offset.unwrap();
		let line = read_line_at(&self.index, offset);

		let line = String::from_utf8(line).unwrap();

		let sections : Vec<&str>= line.split(' ').collect();

		let mut index = 2;

		let synset_cnt : u32 = sections[index].parse().unwrap();

		index += 1;
		let ptr_symbols_cnt : usize = sections[index].parse().unwrap();
		index += 1;

		index += ptr_symbols_cnt;

		index += 1; // skip sense_cnt
		index += 1; // skip tagsense_cnt

		let mut senses = vec!();
		senses.reserve(synset_cnt as usize);

		for synset in 0..synset_cnt
		{
			let synset_offset =
				u64::from_str_radix(sections[index+synset as usize], 10).unwrap();
			senses.push( self.read_sense( database, synset_offset ) );
			// self.read_sense( synset_offset );
		}

		Some(senses)
	}

}


/// Represents a Wordnet database directory
#[derive(Debug)]
pub struct Database
{
	db_files: Vec<DBFile>,
}

impl Database
{
	/// Open a wordnet database directory (not included)
	///
	/// On Debian, these files are present in `/usr/share/wordnet`
	/// and can be installed from the package `wordnet-base`.
	pub fn open(path : &std::path::Path)
		-> std::io::Result<Database>
	{
		let mut db = Database { db_files: vec!() };

		for e in std::fs::read_dir(path)?
		{
			let entry = e?;
			let path_buf = entry.path();
			if path_buf.file_stem().unwrap_or(std::ffi::OsStr::new(""))
				== std::ffi::OsStr::new("index")
			{
				let ex = path_buf.extension().ok_or(std::io::Error::new(
					std::io::ErrorKind::InvalidData,
					"file with invalid part of speech".to_string()
				))?;
				let part_of_speech
					= if ex == "noun"
						{ PartOfSpeech::Noun }
					else if ex == "verb"
						{ PartOfSpeech::Verb }
					else if ex == "adv"
						{ PartOfSpeech::Adverb }
					else if ex == "adj"
						{ PartOfSpeech::Adjective }
					else
					{
						return Err(std::io::Error::new(
							std::io::ErrorKind::InvalidData,
							"file with invalid part of speech"
						));
					};

				let mut data_path = path_buf.with_file_name("data");
				data_path.set_extension(ex);
				db.db_files.push( DBFile::new(
					part_of_speech,
					path_buf.as_path(),
					data_path.as_path(),
				)? );
			}
		}

		if db.db_files.len() == 0
		{
			Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"file with invalid part of speech"
			))
		}
		else
		{
			Ok(db)
		}
	}

	fn dbfile_for_part_of_speech(&self, part_of_speech : &PartOfSpeech)
		-> &DBFile
	{
		for ref db in &self.db_files
		{
			if db.part_of_speech == *part_of_speech
			{
				return db;
			}
		}
		panic!("part of speech file not found {:?}", part_of_speech);
	}

	/// find all senses of a word.
	///
	/// This search is case-insensitive.
	pub fn senses(&self, word : &str)
		-> Vec<Sense>
	{
		let mut all = vec!();
		for w in &self.db_files
		{
			if let Some(x) = w.senses(
				self,
				word
					.to_lowercase()
					.chars()
					.map(|x| if x==' ' { '_' } else { x })
					.collect::<String>()
					.as_bytes()
			)
			{
				all.extend(x);
			}
		}
		all
	}
}

#[cfg(test)]
mod test
{
	#[test]
	fn test_1()
	{
		let wn = ::Database::open(&::std::path::Path::new("/usr/share/wordnet")).unwrap();
		assert_eq!(18, wn.senses("bank").len());
		assert_eq!(
			1,
			wn.senses("bank")[2].pointers
				.iter()
				.filter(|&x| x.relationship == ::Relationship::Hypernym)
				.count()
		);
		assert_eq!(13, wn.senses("thrust").len());
		assert_eq!(3, wn.senses("enlightenment").len());
	}
}
