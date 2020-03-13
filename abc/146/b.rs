// https://atcoder.jp/contests/abc146/submissions/10807367

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
  let n: u8 = read();
  let s: String = read();
  println!("{}",
    s.chars().map(|c|((c as u8 - 65 + n) % 26 + 65) as char)
    .collect::<String>()
  );
}
