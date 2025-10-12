use std::io::BufRead;
use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let seeds = lines.next().expect("seeds").split_whitespace()
    .skip(1)
    .map(|w| w.parse::<u64>().expect("num"))
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
    .map(|&s| maps.iter().fold(s, |s, m|
      m.range((Unbounded, Included(s)))
        .next_back()
        .and_then(
          |(b, (d, l))| if s < b + l { Some(d + s - b) } 
                        else { None })
        .unwrap_or(s)))
    .min()
    .expect("location");
 
  println!("{}", n);
}