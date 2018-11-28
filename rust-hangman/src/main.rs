extern crate yansi;

mod file;
mod game;
mod hangman;

use std::process::Command;
use std::io::{self, Write};

// use file::reader;
use game::instance::Game;
use hangman::canvas::draw;

const STARTING_LIVES: u8 = 5;

fn read_guess() -> Option<char> {
  let mut guess = String::new();

  print!("Input your guess: ");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut guess)
    .expect("\nFailed to read line");

  guess.trim().to_lowercase().chars().nth(0)
}

fn clear() {
  let output = Command::new("clear")
    .output()
    .expect("Failed to clear console");
  println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
  // let file_contents = reader::read("poem.txt".to_string());
  let mut game = Game::new(STARTING_LIVES, "hangman".to_string());

  loop {
    let lives_left = game.get_lives();

    clear();
    game.print_word();
    draw(lives_left);

    if lives_left == 0 {
      println!("Game over ☠️");
      println!("Word was: {}", game.get_word());
      break;
    } else if game.has_guessed_all() {
      println!("You win! 🎉");
      break;
    } else {
      println!("Lives left: {}", lives_left);
    }

    match read_guess() {
      Some(letter) => game.discover_letter(letter),
      None => panic!("Error parsing input")
    };
  };
}
