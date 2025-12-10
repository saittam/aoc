use std::{
  collections::{HashMap, HashSet},
  io::BufRead,
};

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
  let w = xrmap.len() as i64;
  let h = yrmap.len() as i64;
  let yr = |y| yrmap[y as usize];
  let xr = |x| xrmap[x as usize];
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
  if winding.abs() != 4 {
    panic!("loop has bad winding number {winding}");
  }

  let red = pos.iter().collect::<HashSet<_>>();
  let red = &red;

  // Compute the set of tiles which are red or green.
  // First, trace the outline of the loop...
  let mut q = Vec::new();
  let mut green = pos
    .iter()
    .copied()
    .zip(pos.iter().copied().cycle().skip(1))
    .fold(
      HashSet::new(),
      |mut green, ((x1, y1), (x2, y2))| {
        let (dx, dy) = (x2 - x1, y2 - y1);
        let (innerx, innery) = (
          -dy.signum() * winding.signum(),
          dx.signum() * winding.signum(),
        );
        for p in rect((x1, y1), (x2, y2)) {
          if p != (x2, y2) && !green.insert(p) {
            panic!("overlap {p:?}");
          }
          q.push((p.0 + innerx, p.1 + innery));
        }
        green
      },
    );

  // ... then floodfill the interior.
  while let Some((x, y)) = q.pop() {
    if green.insert((x, y)) {
      q.extend(rect((x - 1, y - 1), (x + 1, y + 1)));
    }
  }
  let green = &green;

  // Check red-cornered rectangle sizes. Brute force
  // is possible, but unsatisfying. Instead, precompute
  // some data to quickly obtain extent of possible
  // areas given a left corner.

  // End of green stretch in increasing x direction,
  let greenend = (0..h)
    .flat_map(|y| {
      (0..w).rev().scan(None, move |e, x| {
        let p = (x, y);
        *e = green.contains(&p).then(|| e.unwrap_or(x));
        Some((p, *e))
      })
    })
    .filter_map(|(p, e)| e.map(|e| (p, e)))
    .collect::<HashMap<_, _>>();
  let greenend = &greenend;

  // Next red tile position in decreasing x direction.
  let leftred = (0..h)
    .flat_map(|y| {
      (0..w).scan(None, move |lr, x| {
        let p = (x, y);
        *lr = red.contains(&p).then(|| x).or(*lr);
        Some((p, *lr))
      })
    })
    .filter(|(p, _)| green.contains(&p))
    .collect::<HashMap<_, _>>();
  let leftred = &leftred;

  // For each red tile, compute largest area with the red
  // tile on a left corner.
  let n = pos
    .iter()
    .copied()
    .flat_map(|(x1, y1)| {
      // search in both y directions
      [1, -1].iter().flat_map(move |step| {
        std::iter::successors(Some(y1), move |p| {
          Some(p + step)
        })
        .take_while(move |y2| green.contains(&(x1, *y2)))
        // track max width in positive x direction
        .scan(greenend[&(x1, y1)], move |e, y2| {
          *e = i64::min(greenend[&(x1, y2)], *e);
          Some(
            leftred[&(*e, y2)]
              .filter(|x2| *x2 >= x1)
              .map(|x2| (x2, y2)),
          )
        })
        .filter_map(|p| p)
        // compute area size using reverse transformation
        .map(move |(x2, y2)| {
          ((xr(x1) - xr(x2)).abs() + 1)
            * ((yr(y1) - yr(y2)).abs() + 1)
        })
      })
    })
    .max()
    .expect("max");

  println!("{n}");
}
