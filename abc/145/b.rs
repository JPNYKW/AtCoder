// https://atcoder.jp/contests/abc145/submissions/10807428

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
  let n: usize = read();
  let s: String = read();
  
  println!("{}",
    if n % 2 == 1 || &s[0..n/2] != &s[n/2..n] { "No" }
    else { "Yes" }
  );
}
