use std::io::Write;

fn main() {
  let number = match std::env::args().nth(1) {
    Some(n) => n,
    None => {
      std::io::stderr()
        .write(b"Please supply an integer to parse")
        .unwrap();
      return;
    }
  };
  match string_to_integer(&number) {
    Ok(v) => println!("Successfully parsed {}", v),
    Err(s) => println!("Error: {}", s),
  }
}

fn string_to_integer(s: &str) -> Result<i32, &'static str> {
  let value = s.trim();
  let is_negative: bool;
  if value.chars().nth(0).unwrap() == '-' {
    is_negative = true;
  } else {
    is_negative = false;
  }
  let mut places = value.len() as u32;
  let mut number = 0;
  let mut v;
  for i in value.chars() {
    v = match mapping(i) {
      Ok(n) => n,
      Err(s) => {
        return Err(s);
      }
    };
    number += v * 10_i32.pow(places - 1);
    places -= 1;
  }
  if is_negative {
    return Ok(-1 * number);
  }
  return Ok(number);
}

fn mapping(c: char) -> Result<i32, &'static str> {
  return match c {
    '0' => Ok(0),
    '1' => Ok(1),
    '2' => Ok(2),
    '3' => Ok(3),
    '4' => Ok(4),
    '5' => Ok(5),
    '6' => Ok(6),
    '7' => Ok(7),
    '8' => Ok(8),
    '9' => Ok(9),
    _ => Err("Only characters from 0 - 9 allowed"),
  };
}
