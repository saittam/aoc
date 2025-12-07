use std::collections::{HashMap, HashSet};
use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let start = lines
    .by_ref()
    .next()
    .expect("start")
    .chars()
    .position(|c| c == 'S')
    .expect("startpos");

  let splitters = lines
    .map(|l| {
      l.chars()
        .enumerate()
        .filter_map(|(p, c)| (c == '^').then(|| p as i32))
        .collect::<HashSet<_>>()
    })
    .collect::<Vec<_>>();

  let beams = splitters.iter().fold(
    HashMap::from([(start as i32, 1)]),
    |beams, splitters| {
      let mut nbeams = HashMap::new();
      for (p, n) in beams {
        if splitters.contains(&p) {
          *nbeams.entry(p - 1).or_insert(0) += n;
          *nbeams.entry(p + 1).or_insert(0) += n;
        } else {
          *nbeams.entry(p).or_insert(0) += n;
        }
      }
      nbeams
    },
  );

  let n = beams.iter().map(|(_, n)| n).sum::<u64>();

  println!("{n}");
}
