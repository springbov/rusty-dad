extern crate rand;

use self::rand::Rng;

#[derive(Serialize, Deserialize)]
pub struct Dad
{
  age: u8,
  jokes: Vec<String>
}

impl Dad
{
  pub fn new(age: u8, jokes: Vec<String>) -> Self
  {
    Dad {
      age: age,
      jokes: jokes
    }
  }

  pub fn tell_joke(&self) -> &String
  {
    let which_joke = rand::thread_rng()
                          .gen_range(0, self.jokes.len());
    &self.jokes[which_joke]
  }
}
