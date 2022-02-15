use base64;
use hex;

pub fn hexToBase64(hex: &str) -> String {
    base64::encode(hex::decode(hex).unwrap())
}

fn main() {
  use std::io::{stdin, stdout, Write};
  let mut s = String::new();
  print!("Please enter a hex string: ");
  let _=stdout().flush();
  stdin().read_line(&mut s).expect("Not a string");
  if let Some('\n')=s.chars().next_back() {
      s.pop();
  }
  if let Some('\r')=s.chars().next_back() {
      s.pop();
  }
  let t = hexToBase64(&s);
  println!("Base64 conversion is: {}",t);
}