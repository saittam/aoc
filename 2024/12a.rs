use std::io::BufRead;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let grid = lines.enumerate()
    .fold(HashMap::new(),
          |mut grid, (y, l)| {
            grid.extend(l.chars().enumerate().map(
              |(x, c)| ((x as i32, y as i32), c)));
            grid
          });

  let n = grid.keys()
    .scan(HashSet::new(), |done, p| {
      let mut peri = 0;
      let mut area = 0;
      let crop = grid[p];
      let mut queue = VecDeque::from([*p]);
      while let Some((x, y)) = queue.pop_front() {
        if !done.insert((x, y)) {
          continue;
        }
        area += 1;
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
          let np = (x + dx, y + dy);
          if *grid.get(&np).unwrap_or(&' ') == crop {
            queue.push_back(np);
          } else {
            peri += 1;
          }
        }
      }
      Some(peri * area)
    })
    .sum::<usize>();
  
  println!("{n}");
}