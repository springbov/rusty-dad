mod dad;
use dad::*;

fn main() 
{
  let mut jokes = Vec::new();
  jokes.push("Kid: \"But dad?!?!\"\nDad:\"Butt dad? did you just call me BUTT DAD! you're grounded\"".to_owned());
  jokes.push("Hi hungry I'm dad!".to_owned());
  jokes.push("Did you hear about the goldfish that went bankrupt? Now he's a bronze fish!".to_owned());

  let pappy = Dad::new(43, jokes); 

  println!("{}", pappy.tell_joke());
}
