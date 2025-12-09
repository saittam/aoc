use std::{collections::HashSet, io::BufRead};

type Point = (i64, i64);

fn rect(
  (x1, y1): Point,
  (x2, y2): Point,
) -> impl Iterator<Item = Point> {
  let range = |a, b| i64::min(a, b)..=i64::max(a, b);
  range(x1, x2)
    .flat_map(move |x| range(y1, y2).map(move |y| (x, y)))
}

fn dedup<I: Iterator<Item = i64>>(iter: I) -> Vec<i64> {
  let mut v = iter.collect::<Vec<_>>();
  v.sort();
  v.dedup();
  v
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let pos = lines
    .map(|l| {
      let mut ni = l
        .splitn(2, ',')
        .map(|w| w.parse::<i64>().expect("num"));
      (ni.next().expect("x"), ni.next().expect("y"))
    })
    .collect::<Vec<_>>();

  // Transform coirdinates to treat runs of rows and
  // columns with no red tiles as a single item.
  let xrmap = dedup(pos.iter().map(|(x, _)| *x));
  let yrmap = dedup(pos.iter().map(|(_, y)| *y));
  let pos = pos
    .iter()
    .map(|(x, y)| {
      (
        xrmap.iter().position(|rx| rx == x).unwrap() as i64,
        yrmap.iter().position(|ry| ry == y).unwrap() as i64,
      )
    })
    .collect::<Vec<_>>();

  // Check winding direction of the loop.
  let segment_iter = pos
    .iter()
    .copied()
    .zip(pos.iter().copied().cycle().skip(1))
    .map(|((x1, y1), (x2, y2))| (x2 - x1, y2 - y1))
    .map(|(dx, dy)| (dx.signum(), dy.signum()));
  let winding = segment_iter
    .clone()
    .zip(segment_iter.cycle().skip(1))
    .map(|((sx1, sy1), (sx2, sy2))| sx1 * sy2 - sx2 * sy1)
    .sum::<i64>();
  println!("winding {winding}");
  if winding != 4 {
    panic!("bad loop");
  }

  let mut q = Vec::new();
  let mut set = pos
    .iter()
    .copied()
    .zip(pos.iter().copied().cycle().skip(1))
    .fold(
      HashSet::new(),
      |mut set, ((x1, y1), (x2, y2))| {
        let (dx, dy) = (x2 - x1, y2 - y1);
        // TODO winding direction...
        let (fx, fy) = (-dy.signum(), dx.signum());
        for p in rect((x1, y1), (x2, y2)) {
          if p != (x2, y2) && !set.insert(p) {
            panic!("overlap {p:?}");
          }
          q.push((p.0 + fx, p.1 + fy));
        }
        set
      },
    );

  println!("filling");

  while let Some((x, y)) = q.pop() {
    if set.insert((x, y)) {
      q.extend(rect((x - 1, y - 1), (x + 1, y + 1)));
    }
  }

  println!("testing");

  let n = pos
    .iter()
    .enumerate()
    .flat_map(|(i, p1)| {
      pos.iter().skip(i + 1).map(move |p2| (p1, p2))
    })
    .filter(|(p1, p2)| {
      rect(**p1, **p2).all(|p| set.contains(&p))
    })
    .map(|((x1, y1), (x2, y2))| {
      ((xrmap[*x1 as usize] - xrmap[*x2 as usize]).abs()
        + 1)
        * ((yrmap[*y1 as usize] - yrmap[*y2 as usize])
          .abs()
          + 1)
    })
    .max()
    .expect("max");

  println!("{n}");
}
