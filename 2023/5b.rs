use std::io::BufRead;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};

fn map_min((start, len): (u64, u64),
           maps: &[BTreeMap<u64, (u64, u64)>]) -> u64 {
  let m = if let Some(m) = maps.first() {
    m
  } else {
    return start;
  };

  let lb = m.range((Unbounded, Included(start)))
    .next_back()
    .unwrap_or((&0, &(0, 0)));
  let mut iter = [lb].into_iter()
    .chain(m.range((Included(start), Excluded(start + len))))
    .peekable();
  std::iter::from_fn(|| {
    let e = iter.next()?;
    let ne = iter.peek()
      .map(|(b, _)| **b)
      .unwrap_or(start + len);
    Some((e, ne))
  })
  .map(|((b, (d, l)), n)| (*b, b + l, *d, n))
  .flat_map(|(b, e, d, n)| [(b, e, d), (e, n, e)])
  .filter(|(b, e, _)| b < e && *b <= start + len && *e > start)
  .map(|(l, h, d)| {
    let lc = u64::max(l, start);
    let hc = u64::min(h, start + len);
    (d + lc - l, h - l - (lc - l) - (h - hc))
  })
  .map(|r| map_min(r, &maps[1..]))
  .min()
  .expect("min")
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let l = lines.next().expect("seeds");
  let mut si = l.split_whitespace()
    .skip(1)
    .map(|w| w.parse::<u64>().expect("num"));
  let seeds =
    std::iter::from_fn(|| Some((si.next()?, si.next()?)))
    .collect::<Vec<_>>();
  lines.next();

  let mut maps = Vec::new();
  while lines.next().is_some() {
    maps.push(
      lines.by_ref().take_while(|l| !l.is_empty())
        .map(|l| {
          let mut ni = l.split_whitespace()
            .map(|w| w.parse::<u64>().expect("num"));
          let dest = ni.next().expect("dest");
          (ni.next().expect("source"),
           (dest, ni.next().expect("len")))
        })
        .collect::<BTreeMap<_, _>>());
  }

  let n = seeds.iter()
    .map(|&s| map_min(s, &maps))
    .min()
    .expect("location");
 
  println!("{}", n);
}