use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

enum Tile {
  Empty,
  Horz,
  Vert,
  Up,
  Down,
}

const DIR: [(isize, isize); 4] = [
  (1, 0), (0, 1), (-1, 0), (0, -1),
];

fn energized(map: &HashMap<(isize, isize), Tile>,
             pos: (isize, isize),
             dir: usize) -> usize {
  let mut q = VecDeque::new();
  q.push_back((pos, dir));
  let mut seen = HashSet::new();
  while let Some(((x, y), d)) = q.pop_front() {
    let tile = if let Some(t) = map.get(&(x, y)) {
      t
    } else {
      continue;
    };
  
    if !seen.insert(((x, y), d)) {
      continue;
    }

    let (dx, dy) = DIR[d];
    match tile {
      Tile::Horz if dx == 0 => {
        q.push_back(((x + 1, y), 0));
        q.push_back(((x - 1, y), 2));
      }
      Tile::Vert if dy == 0 => {
        q.push_back(((x, y + 1), 1));
        q.push_back(((x, y - 1), 3));
      }
      Tile::Up => {
        let d = 3 - d;
        let (dx, dy) = DIR[d];
        q.push_back(((x + dx, y + dy), d));
      }
      Tile::Down => {
        let d = (5 - d) % 4;
        let (dx, dy) = DIR[d];
        q.push_back(((x + dx, y + dy), d));
      }
      _ => q.push_back(((x + dx, y + dy), d)),
    }
  }

  seen.iter()
    .map(|(p, _)| p)
    .collect::<HashSet<_>>()
    .len()
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let map = lines
    .enumerate()
    .flat_map(|(y, l)| l.chars().enumerate().filter_map(
      |(x, c)| Some(((x as isize, y as isize), match c {
        '-' => Tile::Horz,
        '|' => Tile::Vert,
        '/' => Tile::Up,
        '\\' => Tile::Down,
        '.' => Tile::Empty,
        _ => panic!("bad tile {}", c)
      }))).collect::<Vec<_>>())
    .collect::<HashMap<_, _>>();

  let w = map.keys().map(|(x, _)| *x).max().expect("w") + 1;
  let h = map.keys().map(|(_, y)| *y).max().expect("h") + 1;

  let n = (0..w).map(|x| ((x, 0), 1)).chain(
    (0..h).map(|y| (((w - 1), y), 2))).chain(
    (0..w).map(|x| ((x, h - 1), 3))).chain(
    (0..h).map(|y| ((0, y), 0)))
    .map(|(p, d)| energized(&map, p, d))
    .max()
    .expect("max");
  
  println!("{}", n);
}