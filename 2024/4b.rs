use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let grid = lines.enumerate()
    .fold(HashMap::new(),
          |mut grid, (y, l)| {
            grid.extend(l.chars().enumerate().map(
              move |(x, c)| ((x as i32, y as i32), c)));
            grid
          });

  let n = grid.iter()
    .filter(|(_, c)| **c == 'A')
    .filter(|((x, y), _)| {
      let g = |dx, dy| grid
        .get(&(x + dx, y + dy))
        .copied()
        .unwrap_or(' ');
      let x = [g(-1, -1), g(1, -1), g(1, 1), g(-1, 1)];
      x.iter().all(|c| *c == 'M' || *c == 'S') &&
      x[0] != x[2] && x[1] != x[3]
    })
    .count();

  println!("{n}");
}