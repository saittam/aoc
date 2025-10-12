use std::io::BufRead;
use std::collections::HashMap;
use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
  Round,
  Square,
  Empty,
}

fn cycle(w: usize, h: usize,
         cur: BTreeMap<(usize, usize), Tile>)
  -> BTreeMap<(usize, usize), Tile> {
  type TF = dyn Fn((usize, usize)) -> (usize, usize);
  [
    (w, h, &(move |(x, y)| (x, y)) as &TF),
    (h, w, &(move |(x, y)| (y, w - 1 - x)) as &TF),
    (w, h, &(move |(x, y)| (w - 1 - x, h - 1 - y)) as &TF),
    (h, w, &(move |(x, y)| (h - 1 - y, x)) as &TF),
  ].into_iter().fold(
    cur,
    |cur, (w, h, trans)| {
      let cur = &cur;
      (0..w).flat_map(
        |x| (0..h).scan(0, move |p, y| {
          let pos = trans((x, y));
          Some(match cur.get(&pos).unwrap_or(&Tile::Empty) {
            Tile::Square => {
              *p = y + 1;
              Some((pos, Tile::Square))
            }
            Tile::Round => {
              *p += 1;
              Some((trans((x, *p - 1)), Tile::Round))
            }
            Tile::Empty => None
          })
        }))
      .filter_map(|e| e)
      .collect::<BTreeMap<_, _>>()
    })
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
    .collect::<BTreeMap<_, _>>();

  let w = map.keys().map(|(x, _)| *x).max().expect("w") + 1;
  let h = map.keys().map(|(_, y)| *y).max().expect("h") + 1;

  const N: usize = 1000000000;
  let mut cur = map.clone();
  let mut seen = HashMap::new();
  for i in 0.. {
    if let Some(pi) = seen.insert(cur.clone(), i) {
      cur = (0..((N - pi) % (i - pi)))
        .fold(cur, |cur, _| cycle(w, h, cur));
      break;
    }

    cur = cycle(w, h, cur);
  }

  let n = cur.iter()
    .filter(|(_, t)| **t == Tile::Round)
    .map(|((_, y), _)| h - y)
    .sum::<usize>();

  println!("{}", n);
}