// https://atcoder.jp/contests/abc148/submissions/10793707

use std::io::*;

fn read() -> usize {
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
  let (a, b) = (read(), read());
  println!("{}", 6 - a - b);
}
