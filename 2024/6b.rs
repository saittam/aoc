use std::io::BufRead;
use std::collections::{HashMap, HashSet};

struct CycleDetect<I: Iterator>
where I::Item: Clone + Eq + std::hash::Hash
{
  iter: I,
  seen: HashSet<I::Item>,
  cycle: bool,
}

impl<I: Iterator> CycleDetect<I> 
where I::Item: Clone + Eq + std::hash::Hash
{
  fn new(iter: I) -> CycleDetect<I> {
    CycleDetect {
      iter,
      seen: HashSet::new(),
      cycle: false,
    }
  }

  fn has_cycle(&mut self) -> bool {
    self.for_each(drop);
    self.cycle
  }
}

impl<I: Iterator> Iterator for CycleDetect<I>
where I::Item: Clone + Eq + std::hash::Hash
{
  type Item = I::Item;

  fn next(&mut self) -> Option<Self::Item> {
    if self.cycle {
      return None;
    }
    let e = self.iter.next()?;
    if !self.seen.insert(e.clone()) {
      self.cycle = true;
      return None;
    }
    Some(e)
  }
}

const DIR: [(i32, i32); 4] =
  [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn walk<'a, Obs>(obs: Obs, start: ((i32, i32), usize))
  -> impl Iterator<Item=((i32, i32), usize)> + use<'a, Obs>
where Obs: Fn((i32, i32)) -> Option<bool>
{
  std::iter::successors(
    Some(start),
    move |((x, y), d)| (0..(DIR.len()))
    .map(move |di| (d + di) % DIR.len())
    .map(|d| (DIR[d], d))
    .map(move |((dx, dy), d)| ((x + dx, y + dy), d))
    .map_while(|(p, d)| obs(p).map(|c| ((p, d), c)))
    .find(|(_, c)| !*c)
    .map(|(e, _)| e))
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut start = None;
  let grid = lines
    .enumerate()
    .fold(HashMap::new(),
          |mut grid, (y, l)| {
            grid.extend(l.chars().enumerate().map(
              |(x, c)| {
                let p = (x as i32, y as i32);
                start = start.or(
                  "^>v<".find(c).map(|d| (p, d)));
                (p, c == '#')
              }));
            grid
          });
  let start = start.expect("start");

  let mut pathi = CycleDetect::new(
    walk(|p| grid.get(&p).copied(), start));
  let n = pathi.by_ref()
    .skip(1)
    .scan((HashSet::new(), start),
          |(seen, prev), (p, d)| {
            let cycle = seen.insert(p) &&
              CycleDetect::new(walk(
                |pt| (pt == p).then_some(true)
                .or(grid.get(&pt).copied()),
                *prev)).has_cycle();
            *prev = (p, d);
            Some(cycle)
          })
    .filter(|c| *c)
    .count();

  assert!(!pathi.has_cycle());

  println!("{n}");
}