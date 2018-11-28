#![allow(dead_code)]

fn if_statement() {
  let temp = 15;
  if temp > 30 {
    println!("really hot outside");
  } else if temp < 10 {
    println!("really cold");
  } else {
    println!("Ok");
  }

  // ternary
  let day = if temp > 20 {"sunny"} else {"cloudy"};
  println!("today is {}", day);

  println!("it is {}",
    if temp > 20 {"sunny"} else if temp < 10 {"cloudy"} else {"OK"});

  println!("it is {}",
    if temp > 20 {
      if temp > 30 {"very hot"} else {"hot"}
    } else if temp < 10 {"cold"} else {"OK"}
  );
}

fn while_and_loop() {
  let mut x = 1;

  while x < 1000 {
    x *= 2;

    if x == 64 {
      continue;
    }

    println!("x = {}", x);
  }

  let mut y = 1;
  loop {
    y *= 2;
    println!("y = {}", y);

    if y == 1 << 10 { break; }
  }
}

fn for_loop() {
  for x in 1..11 {
    if x == 3 {
      continue;
    } else if x == 8 {
      break;
    }

    println!("x = {}", x);
  }

  for (idx, val) in (30..41).enumerate() {
    println!("{}:{}", idx, val);
  }
}

fn match_statement() {
  let country_code = 999; // 1 999

  let country = match country_code {
    44 => "UK",
    46 => "Sweden",
    7 => "Russia",
    1...999 => "unknown",
    _ => "invalid"
  };

  println!("the country with code {} is {}", country_code, country);
}
