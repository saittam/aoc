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

  let n = [(1, -1), (1, 0), (1, 1), (0, 1)].into_iter()
    .map(|(dx, dy)| grid.keys()
        .filter(|(x, y)| !grid.contains_key(&(x - dx,
                                              y - dy)))
        .map(|(mut x, mut y)| {
          let str = std::iter::from_fn(|| {
            let c = grid.get(&(x, y));
            x += dx;
            y += dy;
            c
          }).collect::<String>();
          str.match_indices("XMAS").count() +
          str.match_indices("SAMX").count()
        })
        .sum::<usize>())
    .sum::<usize>();

  println!("{n}");
}