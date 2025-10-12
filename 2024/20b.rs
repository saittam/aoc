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
  let path = std::iter::successors(
    Some((start, None)),
    |((x, y), prev)| DIRS.iter()
    .map(|(dx, dy)| (x + dx, y + dy))
    .filter(|p| grid.contains(&p) && Some(*p) != *prev)
    .next()
    .map(|p| (p, Some((*x, *y)))))
    .enumerate()
    .map(|(i, (p, _))| (p, i))
    .collect::<HashMap<_, _>>();

  let jumps = (-20..=20i32)
    .flat_map(|x| (-20..=20i32).map(move |y| (x, y)))
    .map(|(x, y)| (x, y, (x.abs() + y.abs()) as usize))
    .filter(|(_, _, l)| *l <= 20)
    .collect::<Vec<_>>();
  let n = path.keys()
    .flat_map(|(x, y)| jumps.iter().map(
      move |(dx, dy, l)| ((*x, *y), (x + dx, y + dy), *l)))
    .filter_map(
      |(p, q, l)| path.get(&q)
      .and_then(|d| d.checked_sub(path[&p] + l)))
    .filter(|s| *s >= 100)
    .count();
              
  println!("{n}");
}