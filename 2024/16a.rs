use std::io::BufRead;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let (grid, start, end) = lines
    .enumerate()
    .fold(
      (HashSet::new(), None, None),
      |(mut grid, mut start, mut end), (y, l)| {
        let y = y as i32;
        grid.extend(l.chars().enumerate().filter_map(
          |(x, c)| {
            let pos = (x as i32, y as i32);
            match c {
              '.' => {},
              '#' => return None,
              'S' => start = Some(pos),
              'E' => end = Some(pos),
              _ => panic!("bad cell {c}"),
            }
            Some(pos)
          }));
        (grid, start, end)
      });
  let start = start.expect("start");
  let end = end.expect("end");

  const DIRS: [(i32, i32); 4] =
    [(1, 0), (0, 1), (-1, 0), (0, -1)];
  let mut queue = BinaryHeap::new();
  queue.push(Reverse((0, start, (1, 0))));
  let mut seen = HashSet::new();
  while let Some(Reverse((score, (x, y), dir))) = queue.pop() {
    if (x, y) == end {
      println!("{score}");
      break;
    }

    if !seen.insert(((x, y), dir)) {
      continue;
    }

    queue.extend(
      DIRS.iter()
      .map(|(dx, dy)| ((x + dx, y + dy), (*dx, *dy)))
      .filter(|(p, _)| grid.contains(p))
      .map(|(p, d)| Reverse(
        (score + if d == dir { 1 } else { 1001 }, p, d))));
  }
}