fn main() {

  let print_vector = |x:&Vec<i32>|
  {
    println!("x[0] = {}", x[0]);
  };

  let v = vec![3, 2, 1];
  // borrow
  print_vector(&v);
  println!("x[0] = {}", v[0]);

  let mut a = 40;
  let c = a;

  {
    let b = &mut a; // borrowing mutability
    *b += 2;
  }

  println!("a = {}", a);

  let mut z = vec![3, 2, 1];

  for i in &z {
    println!("i = {}", i);
    // z.push(5); // defended
  }
}
