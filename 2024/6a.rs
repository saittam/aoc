use std::io::BufRead;
use std::collections::HashMap;
use std::ops::ControlFlow::{Break, Continue};

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut start = None;
  let grid = lines.enumerate()
    .fold(HashMap::new(),
          |mut grid, (y, l)| {
            grid.extend(l.chars().enumerate().map(
              |(x, c)| {
                let p = (x as i32, y as i32);
                start = start.or(
                  "^>v<".find(c).map(|d| (p, d)));
                (p, c == '#')
              }));
            grid
          });

  const DIR: [(i32, i32); 4] =
    [(0, -1), (1, 0), (0, 1), (-1, 0)];
  let (Break(visited) | Continue(visited)) =
    std::iter::successors(
      start,
      |((x, y), d)| (0..(DIR.len()))
      .map(|di| (d + di) % DIR.len())
      .map(|d| (DIR[d], d))
      .map(|((dx, dy), d)| ((x + dx, y + dy), d))
      .map_while(|(p, d)| grid.get(&p).map(|c| ((p, d), c)))
      .find(|(_, c)| !**c)
      .map(|(e, _)| e))
    .try_fold(HashMap::new(),
              |mut m, (p, d)| {
                let mut val = d;
                let e = m.entry(p).or_insert(DIR.len());
                std::mem::swap(e, &mut val);
                if val == d { Break(m) } else { Continue(m) }
              });

  println!("{}", visited.len());
}