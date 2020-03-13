// https://atcoder.jp/contests/abc149/submissions/10792121

use std::io::*;
use std::str::FromStr;
use std::cmp;

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
  let (s, t): (String, String) = (read(), read());
  println!("{}{}", t, s);
}
