use std::io::BufRead;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Tile {
  Path,
  Letter(char),
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let map = lines
    .enumerate()
    .flat_map(|(y, l)| l.chars().enumerate().filter_map(
      move |(x, c)| Some(((x + 1, y + 1), match c {
        '-'|'|'|'+' => Tile::Path,
        c if c.is_alphabetic() => Tile::Letter(c),
        _ => return None,
      }))).collect::<Vec<_>>())
    .collect::<HashMap<_, _>>();

  let mut pos = *map.iter()
    .min_by_key(|((_, y), _)| y)
    .expect("start").0;
  let mut dir = 0;
  let len = std::iter::from_fn(move || {
      let (x, y) = pos;
      let neigh = [
        (x, y + 1), (x - 1, y), (x, y - 1), (x + 1, y)
      ];
      let (d, n, t) = [0, 1, 3].into_iter()
        .map(|k| (dir + k) % 4)
        .map(|d| (d, neigh[d]))
        .find_map(|(d, n)| Some((d, n, map.get(&n)?)))?;
      pos = n;
      dir = d;
      Some(*t)
    })
    .count();

  println!("{}", len + 1);
}