extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod dad;
mod get_file;

use dad::*;
use get_file::*;

use std::io;

fn main() 
{
  println!("Where is your dad.json?");
  match read_dad_file(&read_line())
  {
    Err(x) => println!("Awe crap: {}", x),
    Ok(daddy) => {
      println!("{}", daddy.tell_joke());
    }
  };

}

fn read_line() -> String
{
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("GAh, something went wrong while reading from standard in!");

  input
    .trim()
    .to_owned()
}
