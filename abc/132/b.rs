// https://atcoder.jp/contests/abc132/submissions/10772141

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
  let mut p: Vec<usize> = Vec::new();
  for _ in 0..n { p.push(read()); }
  
  let mut x = 0;
  for i in 1..n-1 {
    if 
      (p[i - 1] > p[i] && p[i] > p[i + 1]) || 
      (p[i - 1] < p[i] && p[i] < p[i + 1]) {
      x += 1;
    }
  }
  
  println!("{}", x);
}
