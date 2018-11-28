#![allow(dead_code)]

use std::mem;

const MEANING_OF_LIFE:u8 = 42; // no fixed address

static mut Z:i32 = 123; // static mut - memory unsafe (use unsafe block)

fn operators() {
  // arithmetic

  let mut a = 2 + 3 * 4;
  println!("a = {}", a);

  a += 1; // no ++, --

  println!("remainder of {} / {} = {}", a, 4, (a%4));

  let a_cubed = i32::pow(a, 3);
  println!("{} cubed is {}", a, a_cubed);

  let b = 2.5;
  let b_cubed = f64::powi(b, 3);
  let b_to_pi = f64::powf(b, std::f64::consts::PI);
  println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);


  // bitwise
  let c = 1 | 2; // 01 OR 10 == 11 == 3_10
  println!("1|2 = {}", c);

  let two_to_10 = 1 << 10;
  println!("2^10 = {}", two_to_10);

  // logical
  // let pi_less_4 = std::f64::consts::PI < 4.0; // true
  // let x = 5;
  // let x_is_5 = x == 5;
}

fn types() {
  // unsigned 0...255
  let a:u8 = 123;
  println!("a = {}", a);

  // signed mutable
  let mut b:i8 = 0;
  println!("b = {}", b);
  b = 43;
  println!("b = {}", b);

  // 32-bit signed (i32)
  let mut c = 123456789;
  println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
  c = -1;
  println!("c = {} after modification", c);

  // i8 u8 i16 u16 i32 u32 i64 u64

  // isize - pointer sized
  let z:isize = 123;
  let size_of_z = mem::size_of_val(&z);
  println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

  // characters
  let d:char = 'x';
  println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

  // double precision f64, f32
  let e:f64 = 2.5;
  println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

  // boolean
  let g = false;
  println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}

fn main() {
  unsafe {
    println!("{}", Z);
  }
}
