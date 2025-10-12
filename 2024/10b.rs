use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let (grid, peaks) = lines.enumerate()
    .fold((HashMap::new(), Vec::new()),
          |(mut grid, mut peaks), (y, l)| {
            let hi = l.chars().enumerate().map(
              |(x, c)| ((x as i32, y as i32),
                        c.to_digit(10).expect("num")));
            grid.extend(hi.clone());
            peaks.extend(hi
                         .filter(|(_, h)| *h == 9)
                         .map(|(p, _)| p));
            (grid, peaks)
          });

  let mut tm = peaks.iter()
    .map(|p| (*p, 1))
    .collect::<HashMap<_, _>>();
  for h in (0..9).rev() {
    tm = tm.into_iter().fold(
      HashMap::new(),
      |mut tm, ((x, y), n)| {
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
          let np = (x + dx, y + dy);
          if grid.get(&np) == Some(&h) {
            *tm.entry(np).or_insert(0) += n;
          }
        }
        tm
      });
  }

  let n = tm.values().sum::<usize>();

  println!("{n}");
}