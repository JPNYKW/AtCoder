// https://atcoder.jp/contests/abc147/submissions/10802770

use std::io::*;

fn read() -> String {
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
  let s = read();
  let l = s.len();
  let mut c = 0;
  
  for i in 0..l {
    let t = s.chars().nth(i).unwrap();
    let u = s.chars().nth(l - i - 1).unwrap();
    if t != u { c += 1; }
  }
  
  println!("{}", c / 2);
}
