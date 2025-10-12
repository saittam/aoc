use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let reports = lines
    .map(|l| l.split_whitespace()
         .map(|w| w.parse::<i32>().expect("num"))
         .collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let n = reports.iter()
    .filter(|r| {
      let (l, h) = r.windows(2)
        .map(|w| w[1] - w[0])
        .fold((i32::MAX, i32::MIN),
              |(l, h), d| (i32::min(l, d), i32::max(h, d)));
      l.signum() == h.signum() &&
        (1..=3).contains(&l.abs()) &&
        (1..=3).contains(&h.abs())
    })
    .count();

  println!("{n}");
}