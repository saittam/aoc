use std::io::BufRead;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let bytes = lines
    .map(|l| {
      let mut ni = l
        .split(',')
        .map(|w| w.parse::<i32>().expect("num"));
      (ni.next().expect("x"), ni.next().expect("y"))
    })
    .collect::<Vec<_>>();

  let blocked = bytes.iter()
    .take(1024)
    .collect::<HashSet<_>>();

  const DIRS: [(i32, i32); 4] =
    [(1, 0), (0, 1), (-1, 0), (0, -1)];
  const D: i32 = 70;
  let mut queue = BinaryHeap::new();
  queue.push(Reverse((D + D, (0, 0), 0)));
  let mut seen = HashSet::new();
  while let Some(Reverse(popped)) = queue.pop() {
    let (_, (x, y), l) = popped;
    if (x, y) == (D, D) {
      println!("{l}");
      break;
    }

    if !seen.insert((x, y)) {
      continue;
    }

    queue.extend(
      DIRS.iter()
      .map(|(dx, dy)| (x + dx, y + dy))
      .filter(
        |(x, y)| *x >= 0 && *y >= 0 && *x <= D && *y <= D)
      .filter(|p| !blocked.contains(p))
      .map(|(x, y)| Reverse((l + (D - x) + (D - y),
                            (x, y), l + 1))));
  }
}