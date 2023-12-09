use std::collections::BTreeMap;
use std::collections::HashSet;
use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let n = lines
    .filter(|l| {
      l.split_whitespace()
        .map(|w| {
          w.chars().fold(BTreeMap::new(), |mut f, c| {
            *f.entry(c).or_insert(0) += 1;
            f
          })
        })
        .scan(HashSet::new(), |m, f| Some(m.insert(f)))
        .all(|n| n)
    })
    .count();

  println!("{}", n);
}
