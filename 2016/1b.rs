use std::io::BufRead;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let line = lines.next().expect("input");
  let dirs = line
    .split(',')
    .map(|w| {
      let mut ci = w.trim().chars();
      let d = match ci.next().expect("dir") {
        'L' => 3,
        'R' => 1,
        _ => panic!("dir"),
      };
      (d, ci.as_str().parse::<usize>().expect("steps"))
    })
    .collect::<Vec<_>>();

  const DELTA: [(i64, i64); 4] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
  ];

  let mut visited = HashSet::new();
  for p in dirs.iter()
    .flat_map(|(d, s)| std::iter::once(*d)
              .chain(std::iter::repeat(0).take(s - 1)))
    .scan(((0, 0), 0), |((x, y), d), t| {
      *d = (*d + t) % 4;
      let (dx, dy) = DELTA[*d];
      (*x, *y) = (*x + dx, *y + dy);
      Some((*x, *y))
    }) {
    if !visited.insert(p) {
      println!("{}", p.0.abs() + p.1.abs());
      break;
    }
  }
}