use std::io::BufRead;
use std::collections::HashMap;

fn sym_axes<F, T>(w: usize, h: usize, f: F) -> Vec<usize>
where F: Fn((usize, usize)) -> T, T: Eq {
  (0..h).fold(
    (1..w).collect::<Vec<_>>(),
    |sa, y| sa.into_iter()
    .filter(|x| (0..*x).rev().zip(*x..w)
            .all(|(x1, x2)| f((x1, y)) == f((x2, y))))
    .collect::<Vec<_>>())
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut patterns = Vec::new();
  loop {
    let p = lines.by_ref()
      .take_while(|l| !l.is_empty())
      .enumerate()
      .flat_map(
        |(y, l)| l.chars()
        .enumerate()
        .map(|(x, c)| ((x, y), c))
        .collect::<Vec<_>>())
      .collect::<HashMap<_, _>>();

    if p.is_empty() {
      break;
    }

    let w = p.keys().map(|(x, _)| *x).max().expect("width");
    let h = p.keys().map(|(_, y)| *y).max().expect("height");
    patterns.push((p, w + 1, h + 1));
  }

  let n = patterns.iter()
    .map(|(p, w, h)|
         sym_axes(*w, *h, |(x, y)| p.get(&(x, y)))
         .into_iter().sum::<usize>() +
         sym_axes(*h, *w, |(x, y)| p.get(&(y, x)))
         .into_iter().map(|y| y * 100).sum::<usize>()
    )
    .sum::<usize>();
  
  println!("{}", n);
}