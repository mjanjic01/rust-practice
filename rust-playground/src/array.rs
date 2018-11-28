#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::mem;

fn arrays() {
  let mut a:[i32;5] = [1 ,2 ,3, 4, 5];

  println!("a has {} elments, first is {}",
    a.len(), a[0]
  );

  a[0] = 321;
  println!("a[0] = {}", a[0]);
  println!("{:?}", a);

  if a != [1, 2, 3, 4, 5] {
    println!("does not match");
  }

  let b = [1u16; 10];
  for i in 0..b.len() {
    println!("b[{}] = {}", i, b[i]);
  }

  println!("b took up {} bytes", mem::size_of_val(&b));

  let mtx:[[f32; 3]; 2] = [
    [1.0, 0.0, 0.0],
    [0.0, 2.0, 0.0]
  ];

  println!("{:?}", mtx);

  for i in 0..mtx.len() {
    for j in 0..mtx[i].len() {
      println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
    }
  }
}

fn vectors() {
  let mut a = Vec::new();
  a.push(1);
  a.push(2);
  a.push(3);

  println!("{:?}", a);
  a.push(44);
  println!("{:?}", a);

  // *usize* isize
  // let idx:usize = 111;

  // println!("a[0] = {}", a[idx]);

  match a.get(3) {
    Some(x) => println!("a[6] = {}", x),
    None => println!("No such element")
  }

  for x in &a {
    println!("{}", x);
  }

  a.push(77);
  println!("{:?}", a);

  // Some(77)
  let last_element = a.pop(); // Option
  println!("last elem is {:?}, a = {:?}", last_element, a);

  while let Some(x) = a.pop() {
    println!("{}", x);
  }

  match a.pop() {
    Some(x) => println!("last element is {}", x),
    None => println!("Vector is empty")
  }
}

fn use_slice(slice: &mut[i32]) {
  println!("first elem = {}, len = {}", slice[0], slice.len());
  slice[0] = 1234;
}

fn slices() {
  let mut data = [1, 2, 3, 4, 5];

  use_slice(&mut data[1..4]);
  // use_slice(&mut data);
  println!("{:?}", data);
}

fn strings() {
  // utf-8
  let s:&'static str = "hello there!"; // &str = string slice

  for c in s.chars().rev() {
    println!("{}", c);
  }

  if let Some(first_char) = s.chars().nth(0) {
    println!("first letter is {}", first_char);
  }

  // String - heap
  let mut letters = String::new();
  let mut a = 'a' as u8;
  while a <= ('z' as u8) {
    letters.push(a as char);
    letters.push_str(",");
    a += 1;
  }

  println!("{}", letters);

  // &str <> String
  let u:&str = &letters;

  // concatenation
  //String + str

  let mut abc = String::from("hello world");
  abc.remove(0);
  abc.push_str("!!!");

  println!("{}", abc.replace("ello", "goodbye"));
}

fn sum_and_product(x:i32, y:i32) -> (i32, i32) {
  (x + y, x * y)
}

fn tuples() {
  let x = 3;
  let y = 4;
  let sp = sum_and_product(x, y);

  println!("sp = {:?}", sp);
  println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

  let (a, b) = sp;
  println!("a = {}, b = {}", a, b);

  let sp2 = sum_and_product(4, 7);
  let combined = (sp, sp2);
  println!("{:?}", combined);
  println!("last elem = {}", (combined.1).1);

  let ((c, d), (e, f)) = combined;
  let foo = (true, 42.0, -1i8);
  println!("{:?}", foo);

  let meaning = (42,); // single element tuple
  println!("{:?}", meaning);
}

fn main() {
  // arrays();
  // vectors();
  // slices();
  // strings();
  tuples();
}
