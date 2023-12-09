use std::io::BufRead;

fn tls(s: &str) -> bool {
  s.chars().fold(((None, None, None), 0, 0), |(p, n, m), c| {
    let nn = match c {
      '[' => n + 1,
      ']' => n - 1,
      _ => n,
    };
    let abba = p.0 == Some(c) && p.1 == p.2 && p.0 != p.2;
    let am = if abba { n + 1 } else { 0 };
    ((p.1, p.2, Some(c)), nn, usize::max(m, am))
  }).2 == 1
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let n = lines
    .filter(|s| tls(s))
    .count();

  println!("{}", n);
}