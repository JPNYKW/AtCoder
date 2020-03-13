// https://atcoder.jp/contests/abc148/submissions/10802356

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
  let (s, t): (String, String) = (read(), read());
  
  let mut u = String::new();
  for i in 0..n {
    let si = s.chars().nth(i).unwrap();
    let ti = t.chars().nth(i).unwrap();
    u = format!("{}{}{}", u, si, ti);
  }
  
  println!("{}", u);
}
