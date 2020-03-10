// https://atcoder.jp/contests/abc157/submissions/10735936

macro_rules! input {
  (source = $s:expr, $($r:tt)*) => {
    let mut iter = $s.split_whitespace();
    input_inner!{iter, $($r)*}
  };
  ($($r:tt)*) => {
    let mut s = {
      use std::io::Read;
      let mut s = String::new();
      std::io::stdin().read_to_string(&mut s).unwrap();
      s
    };
    let mut iter = s.split_whitespace();
    input_inner!{iter, $($r)*}
  };
}

macro_rules! input_inner {
  ($iter:expr) => {};
  ($iter:expr, ) => {};

  ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
    let $var = read_value!($iter, $t);
    input_inner!{$iter $($r)*}
  };
}

macro_rules! read_value {
  ($iter:expr, ( $($t:tt),* )) => {
    ( $(read_value!($iter, $t)),* )
  };

  ($iter:expr, [ $t:tt ; $len:expr ]) => {
    (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
  };

  ($iter:expr, chars) => {
    read_value!($iter, String).chars().collect::<Vec<char>>()
  };

  ($iter:expr, usize1) => {
    read_value!($iter, usize) - 1
  };

  ($iter:expr, $t:ty) => {
    $iter.next().unwrap().parse::<$t>().expect("Parse error")
  };
}

fn main() {
  input! {
    a: [[i32; 3]; 3],
    n: i32, b: [i32; n]
  }
  
  let mut stat: Vec<Vec<bool>> = vec![vec![false; 3]; 3];
  for i in 0..3 {
    for j in 0..3 {
      if b.contains(&a[i][j]) {
          stat[i][j] = true;
      }
    }
  }
  
  if stat.contains(&vec![true; 3]) {
    println!("Yes");
    return;
  }
  
  for i in 0..3 {
    if !vec![stat[0][i], stat[1][i], stat[2][i]].contains(&false) {
      println!("Yes");
      return;
    }
  }
  
  if
    stat[1][1] &&
    (!vec![stat[0][0], stat[2][2]].contains(&false) ||
    !vec![stat[2][0], stat[2][0]].contains(&false)) {
    println!("Yes");
    return;
  }
  
  println!("No");
}
