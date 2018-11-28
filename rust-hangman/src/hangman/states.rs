use yansi::{Style};

enum Styles {
  Error,
  Success,
  Warn,
}

fn style_value(style: Styles) -> Style {
  match style {
    Styles::Error => Style::red(),
    Styles::Success => Style::green(),
    Styles::Warn => Style::yellow()
  }
}

fn print_styled(string: &'static str, style: Style) {
  println!("{}", style.paint(string));
}

pub fn draw_full() {
  let style = style_value(Styles::Success);
  print_styled("             ", style);
  print_styled("             ", style);
  print_styled("             ", style);
  print_styled("             ", style);
  print_styled("          O  ", style);
  print_styled("         /|\\ ", style);
  print_styled("         / \\ ", style);
}

pub fn draw_4() {
  let style = style_value(Styles::Success);
  print_styled(" _________   ", style);
  print_styled("|            ", style);
  print_styled("|            ", style);
  print_styled("|            ", style);
  print_styled("|         O  ", style);
  print_styled("|        /|\\ ", style);
  print_styled("|        / \\ ", style);
}

pub fn draw_3() {
  let style = style_value(Styles::Warn);
  print_styled(" _________   ", style);
  print_styled("|            ", style);
  print_styled("|            ", style);
  print_styled("|         O  ", style);
  print_styled("|        /|\\ ", style);
  print_styled("|        / \\ ", style);
  print_styled("|        ||| ", style);
}

pub fn draw_2() {
  let style = style_value(Styles::Warn);
  print_styled(" _________   ", style);
  print_styled("|            ", style);
  print_styled("|         O  ", style);
  print_styled("|        /|\\ ", style);
  print_styled("|        / \\ ", style);
  print_styled("|        ||| ", style);
  print_styled("|        ||| ", style);
}

pub fn draw_1() {
  let style = style_value(Styles::Warn);
  print_styled(" _________   ", style);
  print_styled("|         |  ", style);
  print_styled("|         O  ", style);
  print_styled("|        /|\\ ", style);
  print_styled("|        / \\ ", style);
  print_styled("|        ||| ", style);
  print_styled("|        ||| ", style);
}

pub fn draw_0() {
  let style = style_value(Styles::Error);
  print_styled(" _________   ", style);
  print_styled("|         |  ", style);
  print_styled("|         O  ", style);
  print_styled("|        /|\\ ", style);
  print_styled("|        / \\ ", style);
  print_styled("|            ", style);
  print_styled("|            ", style);
}
