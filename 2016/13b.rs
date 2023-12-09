use std::io::BufRead;
use std::collections::VecDeque;
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

  let mut queue = VecDeque::new();
  queue.push_back((0, (1, 1)));
  let mut seen = HashSet::new();
  while let Some((cost, (x, y))) = queue.pop_front() {
    if !seen.insert((x, y)) || cost == 50 {
      continue;
    }
    let neigh = [
      (x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1),
    ];
    queue.extend(neigh.iter().cloned()
      .filter(|(nx, ny)| *nx >= 0 && *ny >= 0)
      .filter(|n| open(*n, num))
      .map(|(nx, ny)| (cost + 1, (nx, ny))));
  }

  println!("{}", seen.len());
}