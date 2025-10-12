use std::io::BufRead;
use std::collections::{HashMap, HashSet};

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
  let _end = end.expect("end");

  const DIRS: [(i32, i32); 4] =
    [(1, 0), (0, 1), (-1, 0), (0, -1)];
  let mut seen = HashSet::new();
  seen.insert(start);
  let path = std::iter::successors(
    Some(start),
    |(x, y)| DIRS.iter()
    .map(|(dx, dy)| (x + dx, y + dy))
    .filter(|p| grid.contains(&p) && seen.insert(*p))
    .next())
    .enumerate()
    .map(|(i, p)| (p, i))
    .collect::<HashMap<_, _>>();

  const JUMPS: [(i32, i32); 8] = [
    (1, 1), (-1, 1), (-1, -1), (1, -1),
    (2, 0), (0, 2), (-2, 0), (0, -2),
  ];
  let n = path.keys()
    .flat_map(|(x, y)| JUMPS.iter().map(
      move |(dx, dy)| ((*x, *y), (x + dx, y + dy))))
    .filter_map(
      |(p, q)| path.get(&q)
      .and_then(|d| d.checked_sub(path[&p] + 2)))
    .filter(|s| *s >= 100)
    .count();
              
  println!("{n}");
}