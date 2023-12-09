use std::io::BufRead;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let s = lines
    .flat_map(|l| l.split(|c: char| !c.is_alphabetic())
                   .filter(|w| !w.is_empty())
                   .map(str::to_owned)
                   .collect::<Vec<_>>())
    .fold(HashSet::new(), |mut s, w| {
      if !s.take(&w).is_some() {
        s.insert(w);
      }
      s
    });
  assert_eq!(s.len(), 1);

  println!("{}", s.iter().next().expect("root"));
}