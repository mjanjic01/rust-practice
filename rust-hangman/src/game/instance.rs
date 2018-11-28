use std::io::{self, Write};

pub struct Game {
  lives: u8,
  word: String,
  discovered_letters: String
}

impl Game {
  pub fn new(lives: u8, word: String) -> Game {
    Game {
      lives: lives,
      word: word,
      discovered_letters: String::new()
    }
  }

  pub fn get_word(&self) -> &str { &self.word }

  pub fn get_lives(&self) -> u8 { self.lives }

  pub fn dec_lives(&mut self) {
    self.lives -= 1;
  }

  pub fn print_word(&self) {
    for letter in self.word.chars() {
      if self.discovered_letters.contains(letter) {
        print!("{} ", letter);
      } else {
        print!("_ ");
      }
    }
    print!("\n");
    io::stdout().flush().unwrap();
  }


  pub fn has_guessed_all(&self) -> bool {
    for letter in self.word.chars() {
      if !self.discovered_letters.contains(letter) {
        return false;
      }
    }

    true
  }

  pub fn discover_letter(&mut self, letter: char){
    if self.discovered_letters.contains(letter) {
      self.dec_lives();
    } else if self.word.contains(letter) {
      self.discovered_letters.push(letter);
    } else {
      self.dec_lives();
    }
  }
}
