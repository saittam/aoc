use std::io::BufRead;
use std::collections::HashSet;

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
    .collect::<HashSet<_>>();

  let updates = lines
    .map(|l| l.split(',')
         .map(|w| w.parse::<u32>().expect("num"))
         .collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let n = updates.iter()
    .filter(|u| u.iter()
            .enumerate()
            .all(|(i, a)| u.iter()
                 .skip(i + 1)
                 .all(|b| !order.contains(&(*b, *a)))))
    .map(|u| u[u.len() / 2])
    .sum::<u32>();

  println!("{n}");
}