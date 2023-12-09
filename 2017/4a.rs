use std::io::BufRead;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let n = lines
    .filter(|l| l.split_whitespace()
                 .scan(HashSet::new(), |m, w| Some(m.insert(w)))
                 .all(|n| n))
    .count();

  println!("{}", n);
}