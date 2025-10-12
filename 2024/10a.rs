use std::io::BufRead;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let (grid, heads) = lines.enumerate()
    .fold((HashMap::new(), Vec::new()),
          |(mut grid, mut heads), (y, l)| {
            let hi = l.chars().enumerate().map(
              |(x, c)| ((x as i32, y as i32),
                        c.to_digit(10).expect("num")));
            grid.extend(hi.clone());
            heads.extend(hi
                         .filter(|(_, h)| *h == 0)
                         .map(|(p, _)| p));
            (grid, heads)
          });

  let mut tm = heads.iter()
    .map(|p| (*p, Rc::new(HashSet::from([*p]))))
    .collect::<HashMap<_, _>>();
  for _ in 0..9 {
    tm = tm.into_iter().fold(
      HashMap::new(),
      |mut tm, ((x, y), s)| {
        let h = grid[&(x, y)];
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
          let np = (x + dx, y + dy);
          if grid.get(&np) == Some(&(h + 1)) {
            let e = tm.entry(np).or_insert(Rc::clone(&s));
            if !Rc::ptr_eq(&s, &*e) && s != *e {
              *e = Rc::new(s.union(e).copied().collect());
            }
          }
        }
        tm
      });
  }

  let n = tm.values().map(|s| s.len()).sum::<usize>();

  println!("{n}");
}