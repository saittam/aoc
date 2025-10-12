use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let seqs = lines
    .map(|l| l.split_whitespace()
              .map(|w| w.parse::<i64>().expect("num"))
              .collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let n = seqs.iter()
    .map(|s| {
      let b = s.iter().rev().fold(
        Vec::new(),
        |d, e| d.iter().fold(
          vec![*e],
          |mut d, v| {
            d.push(*d.last().unwrap() - *v);
            d
          }));

      assert_eq!(*b.last().expect("last"), 0);

      b.iter().rev().fold(0, |p, v| v + p)
    })
    .sum::<i64>();

  println!("{}", n);
}