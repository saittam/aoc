use std::io::BufRead;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let cards = lines
    .map(|l| {
      let mut pi = l.split(&[':', '|']).skip(1)
        .map(|p| p.split_whitespace()
                  .map(|w| w.parse::<i32>().expect("num"))
                  .collect::<HashSet<_>>());
      (pi.next().expect("winning"),
       pi.next().expect("nums"))
    })
    .collect::<Vec<_>>();

  let n = cards.iter()
    .map(|(w, n)| (1 << w.intersection(n).count()) >> 1)
    .sum::<usize>();
 
  println!("{}", n);
}