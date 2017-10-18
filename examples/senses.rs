extern crate wordnet;

fn print_indent(indent : u32)
{
  for _ in 0..indent
  {
    print!("    ");
  }
}

fn print_tree(indent : u32, ptr : &wordnet::PointerRef)
{
  if ptr.relationship != wordnet::Relationship::Hypernym
    { return; }

  let sense = ptr.read();
  print_indent(indent);
  println!(
    " => {}",
    sense.synonyms.iter().fold(
      "".to_string(),
      |acc,ref x|
      {
        let s = if acc.len()==0 { "" } else { ", " };
        format!("{}{}{}", acc, s, x.word)
      }
    ),
  );
  for p in &sense.pointers
  {
    print_tree(indent+1, p);
  }

}

fn main()
{
  let wn = wordnet::Database::open(&::std::path::Path::new("/usr/share/wordnet")).unwrap();

  for argument in std::env::args().skip(1)
  {
    let senses = wn.senses(&argument);
    println!("{} has {} senses:", argument, senses.len());
    for (n, ref s) in senses.iter().enumerate()
    {
      println!(
        "  {}. {} ({}): {}",
        n,
        s.synonyms.iter().fold(
          "".to_string(),
          |acc,ref x|
          {
            let s = if acc.len()==0 { "" } else { ", " };
            format!("{}{}{}", acc, s, x.word)
          }
        ),
        s.part_of_speech.short(),
        s.gloss,
      );
      for p in &s.pointers
      {
        print_tree(1, p);
      }
    }
  }

}

