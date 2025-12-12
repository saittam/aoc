use std::{collections::HashSet, io::BufRead};

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut lines = lines.peekable();
  let shapes = std::iter::from_fn(|| {
    lines.peek()?.ends_with(":").then(|| {
      lines
        .by_ref()
        .skip(1)
        .take_while(|l| !l.trim().is_empty())
        .enumerate()
        .fold(HashSet::new(), |mut p, (y, l)| {
          p.extend(l.chars().enumerate().filter_map(
            move |(x, c)| (c == '#').then_some((x, y)),
          ));
          p
        })
    })
  })
  .collect::<Vec<_>>();

  let regions = lines
    .map(|l| {
      let mut ni = l
        .split(|c: char| !c.is_digit(10))
        .filter(|w| !w.is_empty())
        .map(|w| w.parse::<usize>().expect("num"));
      let w = ni.next().expect("w");
      let h = ni.next().expect("h");
      let counts = ni.collect::<Vec<_>>();
      ((w, h), counts)
    })
    .collect::<Vec<_>>();

  let shape_width = shapes
    .iter()
    .flat_map(|s| s.iter().map(|(x, _)| x))
    .max()
    .expect("xmax")
    + 1;
  let shape_height = shapes
    .iter()
    .flat_map(|s| s.iter().map(|(x, _)| x))
    .max()
    .expect("ymax")
    + 1;

  let n = regions
    .iter()
    .filter(|((w, h), counts)| {
      let shape_total = counts
        .iter()
        .enumerate()
        .map(|(i, n)| n * shapes[i].len())
        .sum::<usize>();
      if shape_total > w * h {
        return false;
      }

      let nshapes = counts.iter().sum::<usize>();
      let slots = (w / shape_width) * (h / shape_height);
      if nshapes <= slots {
        return true;
      }

      panic!("too hard to solve...");
    })
    .count();

  println!("{n}");
}
