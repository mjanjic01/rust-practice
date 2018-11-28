extern crate rand;
extern crate phrases;

use rand::Rng;
use phrases::greetings::{french, english};

fn main() {
  // let mut rng = rand::thread_rng();
  // let b:bool = rng.gen();
  println!("English: {}, {}", english::hello(), english::goodbye());
  println!("French: {}, {}", french::hello(), french::goodbye());
}
