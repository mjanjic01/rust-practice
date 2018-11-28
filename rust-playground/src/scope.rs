use std::mem;

fn scope_and_shadowing() {
  let a = 123;
  let a = 1234;

  {
    let b = 456;
    println!("inside, b = {}", b);

    let a = 777;
    println!("inside, a = {}", a);
  }

  println!("outsinde, a = {}", a);
}

fn main() {
  // operators();
  scope_and_shadowing();
}
