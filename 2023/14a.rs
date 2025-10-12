use std::io::BufRead;
use std::collections::HashMap;

enum Tile {
  Round,
  Square,
  Empty,
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let map = lines
    .enumerate()
    .flat_map(|(y, l)| l.chars().enumerate().map(
      |(x, c)| ((x, y), match c {
        'O' => Tile::Round,
        '#' => Tile::Square,
        '.' => Tile::Empty,
        _ => panic!("bad tile {}", c)
      })).collect::<Vec<_>>())
    .collect::<HashMap<_, _>>();

  let w = map.keys().map(|(x, _)| *x).max().expect("w") + 1;
  let h = map.keys().map(|(_, y)| *y).max().expect("h") + 1;

  let n = (0..w).map(
    |x| (0..h).fold(
      (0, 0),
      |(w, p), y| match map[&(x, y)] {
        Tile::Square => (w, y + 1),
        Tile::Round => (w + (h - p), p + 1),
        Tile::Empty => (w, p),
      }).0)
    .sum::<usize>();
                     
  println!("{}", n);
}