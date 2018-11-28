use hangman::states;

pub fn draw(lives: u8) {
  match lives {
    0 => states::draw_0(),
    1 => states::draw_1(),
    2 => states::draw_2(),
    3 => states::draw_3(),
    4 => states::draw_4(),
    _ => states::draw_full()
  };
}
