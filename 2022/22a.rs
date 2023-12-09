use std::io::BufRead;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Tile {
  Air,
  Rock,
}

#[derive(Clone, Copy, Debug)]
enum Dir {
  R = 0,
  D = 1,
  L = 2,
  U = 3,
}

#[derive(Clone, Copy)]
enum Turn {
  L = 3,
  R = 1,
}

impl Dir {
  fn turn(&self, t: Turn) -> Dir {
    const T: [Dir; 4] = [ Dir::R, Dir::D, Dir::L, Dir::U ];
    T[(*self as usize + t as usize) % 4]
  }
}

fn step(map: &HashMap<(isize, isize), Tile>,
        pos: (isize, isize),
        dir: Dir) -> (isize, isize) {
  const D: [(isize, isize); 4] = [
    (1, 0), (0, 1), (-1, 0), (0, -1)
  ];
  let (dx, dy) = D[dir as usize];
  let (mut x, mut y) = pos;
  x += dx;
  y += dy;
  let (t, pn) = if let Some(t) = map.get(&(x, y)) {
    (t, (x, y))
  } else {
    std::iter::successors(Some(pos),
                          |(x, y)| Some((x - dx, y - dy)))
      .scan((), |_, p| map.get(&p).map(|t| (t, p)))
      .last()
      .expect("wrap")
  };
  match t {
    Tile::Air => pn,
    Tile::Rock => pos,
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let map = lines.by_ref()
    .take_while(|l| l.len() > 0)
    .enumerate()
    .flat_map(|(y, l)|
      l.chars()
        .enumerate()
        .filter_map(move |(x, c)| match c {
          '.' => Some(Tile::Air),
          '#' => Some(Tile::Rock),
          _ => None,
        }.map(|t| ((x as isize + 1, y as isize + 1), t)))
      .collect::<Vec<_>>()
    )
    .collect::<HashMap<_, _>>();

  let startx = map.keys()
    .filter(|(_, y)| *y == 1)
    .map(|(x, _)| *x)
    .min()
    .expect("startx");

  const TC: [char; 2] = ['R', 'L'];
  let path = lines.next().expect("path");
  let steps = path.split(&TC)
    .map(|w| w.parse::<usize>().expect("steps"));
  let dir = std::iter::once(Dir::R).chain(path
    .split(|c: char| c.is_digit(10))
    .filter(|w| w.len() > 0)
    .map(|w| match w {
      "R" => Turn::R,
      "L" => Turn::L,
      _ => panic!("turn {w}"),
    })
    .scan(Dir::R, |d, t| {
      *d = d.turn(t);
      Some(*d)
    }));

  let ((x, y), d) = steps.zip(dir)
    .flat_map(|(s, d)| std::iter::repeat(d).take(s))
    .scan((startx, 1), |p, d| {
      *p = step(&map, *p, d);
      Some((*p, d))
    })
    .last()
    .expect("last");
  println!("{}", 1000 * y + 4 * x + d as isize);
}