// https://atcoder.jp/contests/abc150/submissions/10792090

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
  let (k, x): (i32, i32) = (read(), read());
  println!("{}",
    if 500 * k >= x { "Yes" }
    else { "No" }
  );
}
