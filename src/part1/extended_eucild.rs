fn extended_euclid(a: i32, b: i32) -> Option<(i32, i32, i32)> {
  let mut r1 = a;
  let mut r2 = b;
  let mut s1 = 1;
  let mut s2 = 0;
  let mut t1 = 0;
  let mut t2 = 1;
  let mut r;
  let mut s;
  let mut t;
  let mut q;

  while r2 != 0 {
      q = r1 / r2;
      // gcd 계산
      r = r1 % r2;
      r1 = r2;
      r2 = r;

      // s 계산
      s = s1 - q * s2;
      s1 = s2;
      s2 = s;

      // t 계산
      t = t1 - q * t2;
      t1 = t2;
      t2 = t;
  }

  r = r1;
  s = s1;
  t = t1;
  
  println!("gcd({}, {}) = {}, s = {}, t = {}", a, b, r, s, t);
  println!("{} x {} + {} x {} = {}", a, s, b, t, r);

  if r == 1 {
      if t < 0 {
          t += a;
      }
      Some((a, s, t))
  } else {
      None
  }
}

pub fn main() {
  let inverse = extended_euclid(26, 11);
  if let Some((a, s, t)) = inverse {
      println!("역원: {}", t);
  }
}