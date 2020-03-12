// https://atcoder.jp/contests/abc150/submissions/10772232

use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
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
  let (_, s): (usize, String) = (read(), read());
  let mut c = 0;
  
  for i in 0..s.len()-2 {
    c += if &s[i..i+3] == "ABC" { 1 } else { 0 };
  }
  
  println!("{}", c);
}
