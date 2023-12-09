use std::io::BufRead;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let layers = lines.map(|l| {
      let mut ni = l.split(|c: char| !c.is_numeric())
        .filter_map(|w| w.parse::<usize>().ok());
      (ni.next().expect("layer"), ni.next().expect("depth"))
    })
    .collect::<Vec<_>>();

  let mut queue = BinaryHeap::new();
  for (l, d) in layers {
    let p = 2 * (d - 1);
    queue.push((Reverse((p - l % p) % p), p));
  }

  let mut next = 0;
  loop {
    let (Reverse(n), p) = queue.pop().expect("pop");
    if n > next {
      println!("{}", next);
      break;
    }
    next = n + 1;
    queue.push((Reverse(n + p), p));
  }
}