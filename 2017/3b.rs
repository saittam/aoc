use std::io::BufRead;
use std::collections::HashMap;

type Pos = (i32, i32);

const STEP: [Pos; 4] = [
  (0, -1),
  (-1, 0),
  (0, 1),
  (1, 0),
];

struct SpiralIter {
  x: i32,
  y: i32,
  n: usize,
  count: usize,
}

impl Iterator for SpiralIter {
  type Item = Pos;
  fn next(&mut self) -> Option<Pos> {
    if self.count == 0 {
      self.n += 1;
      self.count = self.n / 2;
    }
    let p = (self.x, self.y);
    let (dx, dy) = STEP[(self.n + 1) % 4];
    self.x += dx;
    self.y += dy;
    self.count -= 1;
    Some(p)
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let k = lines.next().expect("line")
    .parse::<i32>().expect("num");

  let mut m = HashMap::new();
  m.insert((0, 0), 1);
  let (_, v) = SpiralIter { x: 0, y: 0, n: 2, count: 1 }
    .enumerate()
    .scan(m, |m, (n, (x, y))| {
      let val = m[&(x, y)];
      let neigh = [
        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        (x - 1, y),                 (x + 1, y),
        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
      ];
      for n in neigh {
        *m.entry(n).or_insert(0) += val;
      }
      Some((n, val))
    })
    .find(|(_, v)| *v > k)
    .expect("n");

  println!("{}", v);
}