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

  const DIR: [(isize, isize); 4] = [
    (1, 0), (0, 1), (-1, 0), (0, -1),
  ];

  let mut q = VecDeque::new();
  q.push_back(((0, 0), 0));
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

  let n = seen.iter()
    .map(|(p, _)| p)
    .collect::<HashSet<_>>()
    .len();
  
  println!("{}", n);
}
