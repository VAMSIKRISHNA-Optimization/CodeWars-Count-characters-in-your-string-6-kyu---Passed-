
use std::collections::HashMap;

fn count(input: &str) -> HashMap<char, i32> 
{
  if input.is_empty() { return HashMap::new(); }
  else
  {
    let mut char_counts: HashMap<char, i32> = HashMap::new();

    for ch in input.chars() 
    {
        *char_counts.entry(ch).or_insert(0) += 1;
    }
    return char_counts;
  }
  
}

fn main()
{
    println!("{:?}", count("aba"));
}
