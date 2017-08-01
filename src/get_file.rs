use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use Dad;
use serde_json;


pub fn read_dad_file(name: &str) -> Result<Dad, String>
{
    // Create a path to the desired file
    let path = Path::new(name);
    let dad: Result<Dad, String>;

    match File::open(&path) {
      // The `description` method of `io::Error` returns a string that
      // describes the error
      Err(why) => dad = Err(format!("{}", why)),
      Ok(mut file) => {
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Ok(_) => {
              match serde_json::from_str(&s) {
                Ok(x) => dad = Ok(x),
                Err(y) => dad = Err(format!("{}",y))
              }

            },
            Err(why) => dad = Err(format!("{}",why))
        };
      }
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    dad
}
