use std::io::BufRead;
use std::cmp::Ordering;

fn gcd(ai: usize, bi: usize) -> usize {
  let mut a = ai;
  let mut b = bi;
  while b != 0 {
    let t = a % b;
    a = b;
    b = t;
  }
  a
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut tb = String::new();
  handle.read_line(&mut tb);
  let _t = tb.trim().parse::<usize>().unwrap();
  
  let mut bb = String::new();
  handle.read_line(&mut bb);
  let buses = bb.trim().split(',').enumerate()
    .filter(|(_, s)| *s != "x")
    .map(|(m, s)| (m, s.parse::<usize>().unwrap()))
    .collect::<Vec<(usize, usize)>>();
  
  let mut p = 1;
  let mut o = 0;
  for (m, b) in &buses {
    // k1 * p + o == k2 * b - m
    let mut kp = o + *m;
    let mut kb = 0;
    loop {
      match kp.cmp(&kb) {
        Ordering::Less => kp += p * (1 + (kb - kp - 1) / p),
        Ordering::Equal => break,
        Ordering::Greater => kb += b * (1 + (kp - kb - 1) / b),
      }
    }
    p = (p * b) / gcd(p, *b);
    o = kb - *m;
  }

  println!("{}", o);
}