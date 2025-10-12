use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let order = lines.by_ref()
    .take_while(|l| l.len() > 0)
    .map(|l| {
      let mut ni = l.split('|')
         .map(|w| w.parse::<u32>().expect("num"));
      (ni.next().expect("a"), ni.next().expect("b"))
    })
    .fold(HashMap::new(),
          |mut m, (a, b)| {
            *m.entry(b).or_insert(0) |= 1u128 << a;
            m
          });

  let updates = lines
    .map(|l| l.split(',')
         .map(|w| w.parse::<u32>().expect("num"))
         .collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let mut n = 0;
  for u in updates {
    let mut mask = u.iter()
      .fold(0u128, |m, p| m | (1u128 << p));

    let mut seq = Vec::new();
    while mask != 0 {
      let next = u.iter().copied()
        .filter(|p| ((1u128 << p) & mask) != 0)
        .filter(|p| (order.get(p).unwrap_or(&0) & mask) == 0)
        .next()
        .expect("next");
      assert!(((1u128 << next) & mask) != 0);
      seq.push(next);
      mask &= !(1u128 << next);
    }

    if seq != u {
      n += seq[seq.len() / 2];
    }
  }

  println!("{n}");
}