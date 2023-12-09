use std::io::BufRead;

fn ssl(s: &str) -> bool {
  let mut m = s.chars().fold(((None, None), 0, Vec::new()),
                             |(p, n, mut m), c| {
    let nn = match c {
      '[' => n + 1,
      ']' => n - 1,
      _ => n,
    };
    let aba = p.0 == Some(c) && p.1 != p.0;
    if aba {
      let p1 = p.1.expect("middle");
      let hs = n > 0;
      let ac = if hs { (c, p1) } else { (p1, c) };
      m.push((ac, hs));
    }
    ((p.1, Some(c)), nn, m)
  }).2;
  m.sort();
  m.windows(2).any(|w| w[0] == (w[1].0, !w[1].1))
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let n = lines
    .filter(|s| ssl(s))
    .count();

  println!("{}", n);
}