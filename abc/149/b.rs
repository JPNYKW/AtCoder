// https://atcoder.jp/contests/abc149/submissions/10793558

use std::io::*;
use std::cmp;

fn read() -> i64 {
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
  let (a, b, k) = (read(), read(), read());
  println!("{}",
    if k <= a { format!("{} {}", a - k, b) }
    else { format!("0 {}", cmp::max(b - (k - a), 0)) }
  );
}
