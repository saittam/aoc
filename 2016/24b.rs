use std::io::BufRead;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

type Pos = (i32, i32);

fn neigh_pos((x, y): Pos) -> impl IntoIterator<Item = Pos> {
  [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn absdist((ax, ay): &Pos, (bx, by): &Pos) -> u32 {
  ((ax - bx).abs() + (ay - by).abs()) as u32
}

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

  let mut start = None;
  let mut npos = Vec::new();
  let pos = lines
    .enumerate()
    .flat_map(|(y, l)| l.chars()
              .enumerate()
              .filter(|(_, c)| *c == '.' || c.is_digit(10))
              .inspect(|(x, c)|
                       if let Some(n) = c.to_digit(10) {
                         let p = (*x as i32, y as i32);
                         if n == 0 {
                           start = Some(npos.len());
                         }
                         npos.push(p);
                       })
              .map(move |(x, _)| (x as i32, y as i32))
              .collect::<Vec<_>>())
    .collect::<HashSet<_>>();
  let last = npos.len() - 1;
  npos.swap(start.expect("zero"), last);

  let pos = &pos;  // capture by reference
  let mut npi = npos.iter().enumerate();
  let npii = std::iter::from_fn(|| {
    npi.next();
    Some(npi.clone())
  });
  let dist = npos.iter().enumerate().zip(npii)
    .flat_map(|((i1, p1), npi)| npi.flat_map(
      move |(i2, p2)| {
        let d = path_len(
          *p1,
          |p| *p == *p2,
          |p| absdist(p, p2),
          |p, d| neigh_pos(p).into_iter()
            .filter(|n| pos.contains(n))
            .map(move |n| (n, d + 1))
        ).expect("path");
        [((i1, i2), d), ((i2, i1), d)]
      }))
    .collect::<HashMap<_, _>>();

  npos.pop();
  let dist = &dist;
  let tm = (1..npos.len()).fold(
    (0..npos.len())
      .map(|k| ((k, 1 << k), dist[&(k, npos.len())]))
      .collect::<HashMap<_, _>>(),
    |tm, _| {
      let mut ntm = HashMap::new();
      for (ks, l) in (0..npos.len())
        .flat_map(|k|
          tm.iter()
            .filter(move |((_, s), _)| ((1 << k) & *s) == 0)
            .map(move |((i, s), l)| ((k, s | (1 << k)),
                                     l + dist[&(k, *i)]))) {
        let e = ntm.entry(ks).or_insert(l);
        *e = u32::min(*e, l);
      }
      ntm
    });

  let d = tm.iter()
    .map(|((i, _), l)| l + dist[&(*i, npos.len())])
    .min()
    .expect("min");

  println!("{}", d);
}