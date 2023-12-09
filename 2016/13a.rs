use std::io::BufRead;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

fn open((x, y): (isize, isize), n: isize) -> bool {
  let v = x*x + 3*x + 2*x*y + y + y*y + n;
  v.count_ones() % 2 == 0
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let num = lines.next().expect("line")
    .parse::<isize>().expect("num");

  const DEST: (isize, isize) = (31, 39);
  
  let mut queue = BinaryHeap::new();
  queue.push((Reverse(DEST.0 + DEST.1 - 2), 0, (1, 1)));
  let mut seen = HashSet::new();
  while let Some((Reverse(_), cost, (x, y))) = queue.pop() {
    if !seen.insert((x, y)) {
      continue;
    }
    if (x, y) == DEST {
      println!("{}", cost);
      break;
    }
    let neigh = [
      (x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1),
    ];
    queue.extend(neigh.iter().cloned()
      .filter(|(nx, ny)| *nx >= 0 && *ny >= 0)
      .filter(|n| open(*n, num))
      .map(|(nx, ny)| {
        let dist = (nx - DEST.0).abs() + (ny - DEST.1).abs();
        (Reverse(cost + 1 + dist), cost + 1, (nx, ny))
      }));
  }
}