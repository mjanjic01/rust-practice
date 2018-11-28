#![allow(dead_code)]
#![allow(unused_imports)]

union IntOrFloat {
  i: i32,
  f: f32
}

fn process_value(iof: IntOrFloat) {
  unsafe {
    match iof {
      IntOrFloat { i: 42 } => println!("meaning of life"),
      IntOrFloat { f } => println!("f32 = {}", f),
    }
  }
}

fn unions() {
  let mut iof = IntOrFloat { i: 3 };

  iof.i = 42;
  // let value = unsafe { iof.i };

  process_value(iof);
}

fn main() {
  unions();
}
