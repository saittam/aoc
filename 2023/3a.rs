use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let (mp, ms) = lines
    .enumerate()
    .fold(
      (HashMap::new(), HashSet::new()),
      |(mp, ms), (y, l)| {
        let (mp, ms, _) = l.chars()
          .chain(std::iter::once('.'))
          .enumerate()
          .fold(
          (mp, ms, None),
          |(mut mp, mut ms, num), (x, c)| {
            let num = if let Some(d) = c.to_digit(10) {
              num
                .or(Some((x, 0)))
                .map(|(xl, n)| (xl, n * 10 + d))
            } else {
              if let Some((xl, n)) = num {
                mp.insert((xl + 1, x, y + 1), n);
              }
              if c != '.' {
                ms.insert((x + 1, y + 1));
              }
              None
            };
            (mp, ms, num)
          });
          (mp, ms)
      });

  let n = mp.iter()
    .filter(|((xl, xh, y), _)| ((y - 1)..=(y + 1))
            .flat_map(
              |y| ((*xl - 1)..=(*xh + 1)).map(move |x| (x, y)))
            .any(|p| ms.contains(&p)))
    .map(|(_, n)| n)
    .sum::<u32>();
  
  println!("{}", n);
}