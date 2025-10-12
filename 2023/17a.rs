use std::io::BufRead;
use std::cmp::Reverse;
use std::collections::hash_map::Entry;
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
  let dist = target.0 + target.1 - 1;
  
  let mut queue = BinaryHeap::new();
  queue.push((Reverse(dist), (1, 0), 0, 3));
  queue.push((Reverse(dist), (0, 1), 1, 3));
  let mut seen = HashMap::new();
  while let Some((Reverse(d), pos, dir, k)) = queue.pop() {
    let l = if let Some(l) = map.get(&pos) {
      l
    } else {
      continue;
    };

    let d = d + l;
    let (px, py) = pos;

    if pos == target {
      println!("{}", d);
      break;
    }

    let entry = seen.entry((pos, dir));
    match entry {
      Entry::Occupied(mut e) => if *e.get() >= k {
        continue;
      } else {
        *e.get_mut() = k;
      }
      Entry::Vacant(e) => {
        e.insert(k);
        for dirn in [(dir + 1) % 4, (dir + 3) % 4] {
          let (dx, dy) = DIR[dirn];
          let pn = (px + dx, py + dy);
          queue.push((Reverse(d - dx - dy), pn, dirn, 3));
        }
      }
    }

    if k > 1 {
      let (dx, dy) = DIR[dir];
      let pn = (px + dx, py + dy);
      queue.push((Reverse(d - dx - dy), pn, dir, k - 1));
    }
  }
}