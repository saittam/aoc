use std::collections::HashSet;
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

  let n = splitters
    .iter()
    .fold(
      (0, HashSet::from([start as i32])),
      |(n, beams), splitters| {
        let hits = &beams
          .intersection(&splitters)
          .copied()
          .collect::<HashSet<_>>();
        let n = n + hits.len();
        let beams = beams
          .difference(&hits)
          .copied()
          .chain(hits.iter().flat_map(|p| [p - 1, p + 1]))
          .collect::<HashSet<_>>();
        (n, beams)
      },
    )
    .0;

  println!("{n}");
}
