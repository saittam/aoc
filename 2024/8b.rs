use std::io::BufRead;
use std::collections::{HashMap, HashSet};

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let (grid, antennas) = lines.enumerate()
    .fold((HashMap::new(), HashMap::new()),
          |(mut grid, mut antennas), (y, l)| {
            grid.extend(l.chars().enumerate().map(
              |(x, c)| {
                let p = (x as i32, y as i32);
                if c != '.' {
                  antennas.entry(c).or_insert(Vec::new())
                  .push(p);
                }
                (p, c)
              }));
            (grid, antennas)
          });

  let antinodes = antennas.values().flat_map(
    |loc| loc.iter().enumerate().flat_map(
      |(i, p1)| loc.iter().skip(i + 1).map(
        move |p2| (p1, p2)))
    .flat_map(|((x1, y1), (x2, y2))| {
        let (dx, dy) = (x2 - x1, y2 - y1);
        [((*x1, *y1), (dx, dy)), ((*x2, *y2), (-dx, -dy))]
          .iter()
          .flat_map(
            |((x, y), (dx, dy))| std::iter::successors(
              Some((*x, *y)),
              move |(x, y)| Some((x + dx, y + dy)))
            .take_while(|p| grid.get(&p).is_some()))
          .collect::<Vec<_>>()
      }))
    .filter(|p| grid.get(&p).is_some())
    .collect::<HashSet<_>>();

  println!("{}", antinodes.len());
}