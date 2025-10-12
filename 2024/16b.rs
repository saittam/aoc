use std::io::BufRead;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;
use std::cmp::Reverse;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let (grid, start, end) = lines
    .enumerate()
    .fold(
      (HashSet::new(), None, None),
      |(mut grid, mut start, mut end), (y, l)| {
        let y = y as i32;
        grid.extend(l.chars().enumerate().filter_map(
          |(x, c)| {
            let pos = (x as i32, y as i32);
            match c {
              '.' => {},
              '#' => return None,
              'S' => start = Some(pos),
              'E' => end = Some(pos),
              _ => panic!("bad cell {c}"),
            }
            Some(pos)
          }));
        (grid, start, end)
      });
  let start = start.expect("start");
  let end = end.expect("end");

  const DIRS: [(i32, i32); 4] =
    [(1, 0), (0, 1), (-1, 0), (0, -1)];
  
  let mut queue = BinaryHeap::new();
  queue.push(Reverse((0, start, (1, 0), None)));

  let mut min_score = None;
  let mut paths = HashMap::new();
  while let Some(Reverse(popped)) = queue.pop() {
    let (score, (x, y), dir, prev) = popped;
    if min_score.unwrap_or(score) < score {
      break;
    }

    if (x, y) == end {
      min_score = Some(score);
    }

    let entry = paths.entry(((x, y), dir));
    let seen = matches!(entry, Entry::Occupied(_));
    let (escore, links) =
      entry.or_insert((score, Vec::new()));
    if *escore == score {
      links.extend(prev);
    }

    if seen {
      continue;
    }
    
    let neighbors = DIRS.iter()
      .map(|(dx, dy)| ((x + dx, y + dy), (*dx, *dy)))
      .filter(|(p, _)| grid.contains(p));
    for (np, d) in neighbors {
      let score = score + 1 + (d != dir) as usize * 1000;
      let prev = Some(((x, y), dir));
      queue.push(Reverse((score, np, d, prev)));
    }
  }
  assert!(min_score.is_some());

  // Traverse paths reaching the end with min_score.
  let mut queue =
    DIRS.iter().map(|d| (end, *d)).collect::<VecDeque<_>>();
  let mut set = HashSet::new();
  while let Some(e) = queue.pop_front() {
    if set.insert(e) {
      if let Some((_, prev)) = paths.get(&e) {
        queue.extend(prev);
      }
    }
  }

  let n = set.iter()
    .map(|(p, _)| *p)
    .collect::<HashSet<_>>()
    .len();

  println!("{n}");
}