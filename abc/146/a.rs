// https://atcoder.jp/contests/abc146/tasks/abc146_b

use std::io::*;

fn read() -> String {
  let stdin = stdin();
  let stdin = stdin.lock();
  let token: String = stdin
    .bytes()
    .map(|c| c.expect("failed to read char") as char) 
    .skip_while(|c| c.is_whitespace())
    .take_while(|c| !c.is_whitespace())
    .collect();
    
  token.parse().ok().expect("failed to parse token")
}

fn main() {
  let s = read();
  println!("{}",
    match &*s {
      "SUN" => 7,
      "MON" => 6,
      "TUE" => 5,
      "WED" => 4,
      "THU" => 3,
      "FRI" => 2,
      "SAT" => 1,
      _ => 0
    }
  );
}
