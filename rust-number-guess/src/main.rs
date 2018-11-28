extern crate rand;
extern crate yansi;


use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;
use yansi::{Style};


fn main() {
  let info: Style = Style::blue();
  let alert: Style = Style::red().bold().italic();
  let success: Style = Style::green().bold();
  let warn: Style = Style::yellow();

  println!("{}", info.paint("Guess the number!"));
  let secret_number = rand::thread_rng().gen_range(1, 101);

  loop {
    let mut guess = String::new();
    print!("{} ", info.paint("Input your guess:"));
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut guess)
      .expect("\nFailed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => {
        println!("You guessed {}", success.paint(num));
        num
      },
      Err(_) => {
        println!("{}", alert.paint("Please input a number."));
        continue;
      }
    };

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("{}", warn.paint("Too small!")),
      Ordering::Greater => println!("{}", warn.paint("Too big!")),
      Ordering::Equal => {
        println!("{}", success.paint("You win!"));
        break;
      },
    }
  }
}
