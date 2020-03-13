// https://atcoder.jp/contests/abc147/submissions/10802439

use std::io::*;

fn read() -> i8 {
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
 let (a, b, c) = (read(), read(), read());
  println!("{}",
    if a + b + c >= 22 { "bust" }
    else { "win" }
  );
}
