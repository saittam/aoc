use std::io::BufRead;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BinaryHeap;

const DIR: [(isize, isize); 4] = [
  (1, 0), (0, 1), (-1, 0), (0, -1),
];

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let map = lines
    .enumerate()
    .flat_map(|(y, l)| l.chars().enumerate().map(
      |(x, c)| ((x as isize, y as isize),
                c.to_digit(10).expect("digit") as isize))
      .collect::<Vec<_>>())
    .collect::<HashMap<_, _>>();
  
  let target = *map.keys().max().expect("target");

  assert!(map.values().all(|l| *l > 0));
  let dist = target.0 + target.1;
  
  let mut queue = BinaryHeap::new();
  queue.push((Reverse(dist), (0, 0), 0, 10));
  queue.push((Reverse(dist), (0, 0), 1, 10));
  let mut seen = HashSet::new();
  while let Some((Reverse(d), (x, y), dir, k)) = queue.pop() {
    let (dx, dy) = DIR[dir];
    let pos = (x + dx, y + dy);
    
    let l = if let Some(l) = map.get(&pos) {
      l
    } else {
      continue;
    };

    let d = d - dx - dy + l;

    if k <= 7 {
      if pos == target {
        println!("{}", d);
        break;
      }
    
      if seen.insert((pos, dir % 2)) {
        for dirn in [(dir + 1) % 4, (dir + 3) % 4] {
          queue.push((Reverse(d), pos, dirn, 10));
        }
      }
    }

    if k > 1 {
      queue.push((Reverse(d), pos, dir, k - 1));
    }
  }
}