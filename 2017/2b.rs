use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let n = lines
    .map(|l| {
      let nums = l.split_whitespace()
        .filter_map(|w| w.parse::<i32>().ok())
        .collect::<Vec<_>>();
      nums.iter()
        .flat_map(|n| nums.iter().map(|d| (*n, *d)))
        .filter(|(n, d)| n != d)
        .map(|(n, d)| (n / d, n % d))
        .filter(|(_, r)| *r == 0)
        .map(|(q, _)| q)
        .next()
        .expect("quotient ")
    })
    .sum::<i32>();
  
  println!("{}", n);
}