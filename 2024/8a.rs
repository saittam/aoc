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
        let (dx, dy) = (2 * (x2 - x1), 2 * (y2 - y1));
        [(x1 + dx, y1 + dy), (x2 - dx, y2 - dy)]
      }))
    .filter(|p| grid.get(&p).is_some())
    .collect::<HashSet<_>>();

  println!("{}", antinodes.len());
}