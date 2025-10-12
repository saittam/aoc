use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

enum Tile {
  Path,
  Slope(usize),
}

type Pos = (isize, isize);

fn search(map: &HashMap<Pos, Tile>,
          goal: Pos,
          p: Pos,
          l: usize,
          seen: &mut HashSet<Pos>)
  -> Option<usize> {
  let t = map.get(&p)?;
  
  if p == goal {
    return Some(l);
  }

  if !seen.insert(p) {
    return None;
  }
    
  let (x, y) = p;
  let neigh = [
    (x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1),
  ];

  let result = match t {
    Tile::Path => neigh.into_iter()
      .filter_map(|n| search(map, goal, n, l + 1, seen))
      .max(),
    Tile::Slope(d) => search(map, goal, neigh[*d], l + 1, seen)
  };

  seen.remove(&p);
  result
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let map = lines
    .enumerate()
    .flat_map(|(y, l)| l.chars()
      .enumerate()
      .map(move |(x, c)| ((x as isize, y as isize), c))
      .collect::<Vec<_>>())
    .filter_map(|(p, c)| Some((p, match c {
      '.' => Tile::Path,
      '>' => Tile::Slope(0),
      'v' => Tile::Slope(1),
      '<' => Tile::Slope(2),
      '^' => Tile::Slope(3),
      '#' => None?,
      _ => panic!("bad tile {}", c),
    })))
    .collect::<HashMap<_, _>>();

  let start = *map.keys().min().expect("start");
  let goal = *map.keys().max().expect("goal");

  let n = search(&map, goal, start, 0, &mut HashSet::new())
    .expect("no solution");

  println!("{}", n);
}