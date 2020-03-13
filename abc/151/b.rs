// https://atcoder.jp/contests/abc151/submissions/10792023

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
  let (n, k, m): (i32, i32, i32) = (read(), read(), read());
  let mut a: Vec<i32> = Vec::new();
  for _ in 0..n-1 { a.push(read()); }
  let s = n * m - a.iter().sum::<i32>();
  println!("{}",
    if k >= s { format!("{}", cmp::max(s, 0)) }
    else { String::from("-1") }
  );
}
