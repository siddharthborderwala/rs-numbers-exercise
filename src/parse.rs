use std::io::Write;

// only for whole numbers

fn main() {
  let number = match std::env::args().nth(1) {
    Some(n) => n,
    None => {
      std::io::stderr()
        .write(b"Please supply a number to parse")
        .unwrap();
      return;
    }
  };
  println!("{}", string_to_integer(&number));
}

fn string_to_integer(s: &str) -> i32 {
  let value = s.trim();
  let is_negative: bool;
  if value.chars().nth(0).unwrap() == '-' {
    is_negative = true;
  } else {
    is_negative = false;
  }
  let mut places = value.len() as u32;
  let mut number = 0;
  for i in value.chars() {
    number += mapping(i) * 10_i32.pow(places - 1);
    places -= 1;
  }
  if is_negative {
    return -1 * number;
  }
  return number;
}

fn mapping(c: char) -> i32 {
  return match c {
    '0' => 0,
    '1' => 1,
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    _ => 0,
  };
}
