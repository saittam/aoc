use std::io::BufRead;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::collections::BinaryHeap;

type Pos = (i32, i32);

fn neigh_pos((x, y): Pos) -> impl IntoIterator<Item = Pos> {
  [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn absdist((ax, ay): &Pos, (bx, by): &Pos) -> u32 {
  ((ax - bx).abs() + (ay - by).abs()) as u32
}

// to generalize:
// * Pos type param
// * mindist function param
// * neighbor function param
// * termination check instead of goal
fn path_len<P, GF, DF, NF, NI>(
  start: P,
  goal: GF,
  mindist: DF,
  neigh: NF) -> Option<u32>
where
  P: Clone + Eq + std::hash::Hash + std::cmp::Ord + std::fmt::Debug,
  GF: Fn(&P) -> bool,
  DF: Fn(&P) -> u32,
  NF: Fn(P, u32) -> NI,
  NI: Iterator<Item = (P, u32)>,
{
  let mut visited = HashSet::new();
  let mut queue = BinaryHeap::new();
  queue.push((Reverse(mindist(&start)), 0, start.clone()));
  while let Some((_, dist, p)) = queue.pop() {
    if !visited.insert(p.clone()) {
      continue;
    }

    if goal(&p) {
      return Some(dist);
    }

    for (n, d) in neigh(p, dist) {
      let nmd = d + mindist(&n);
      queue.push((Reverse(nmd), d, n));
    }
  }
  None
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let nodes = lines
    .skip(2)
    .map(|l| {
      let n = l.split(|c: char| !c.is_numeric())
        .filter_map(|w| w.parse::<u32>().ok())
        .map(|n| n as i32)
        .collect::<Vec<_>>();
      ((n[0], n[1]), n[3], n[4])
    })
    .collect::<Vec<_>>();

  let (min_use, max_avail) = nodes.iter()
    .fold(((i32::MAX, i32::MAX), (i32::MIN, i32::MIN)),
          |((u1, u2), (a1, a2)), (_, u, a)| {
            let (u1, u12) =
              if *u < u1 { (*u, u1) } else { (u1, *u) };
            let (a1, a12) =
              if *a > a1 { (*a, a1) } else { (a1, *a) };
            ((u1, i32::min(u12, u2)), (a1, i32::max(a12, a2)))
          });

  // Make sure the input guarantees that there's only a
  // single empty node and that the only option is to move
  // data there such that the simplifying assumptions in
  // in the problem description hold.
  assert_eq!(min_use.0, 0);
  assert!(min_use.1 > max_avail.1);
  assert!(max_avail.0 > min_use.1);

  let pos = nodes.iter()
    .filter(|(_, u, _)| *u <= max_avail.0)
    .map(|(p, _, _)| *p)
    .collect::<HashSet<_>>();
  let (empty, _, _) = nodes.iter()
    .find(|(_, u, _)| *u == 0)
    .expect("empty");
  let max_x = nodes.iter()
    .filter(|((_, y), _, _)| *y == 0)
    .map(|((x, _), _, _)| *x)
    .max()
    .expect("max x");

  let pos = &pos;
  let n = path_len(
    ((max_x, 0), *empty),
    |(g, _)| *g == (0, 0),
    |((gx, gy), _)| {
      let diag = i32::min(*gx, *gy);
      let rect = i32::max(gx - diag, gy - diag);
      i32::max(0, 3 * (diag - 1) + 1) as u32 +
      i32::max(0, 5 * (rect - 1) + 1) as u32
    },
    |(g, e), d|
      neigh_pos(g).into_iter()
        .filter(|n| pos.contains(&n))
        .filter_map(move |n| {
          let ed = path_len(
            e,
            |p| *p == n,
            |p| absdist(p, &n),
            |p, d|
              neigh_pos(p).into_iter()
                .filter(|n| pos.contains(n) && *n != g)
                .map(move |n| (n, d + 1)))?;
          Some(((n, g), d + ed + 1))
        })
  );

  println!("{}", n.expect("no solution"));
}