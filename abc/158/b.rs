// https://atcoder.jp/contests/abc158/submissions/10721342

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
  let n: i64 = read();
  let a: i64 = read();
  let b: i64 = read();
  
  let x = n / (a + b) * a;
  let y = n % (a + b);
  println!("{}", x + cmp::min(y, a));
}
