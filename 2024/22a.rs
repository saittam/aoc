use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let nums = lines
    .map(|l| l.parse::<u64>().expect("num"))
    .collect::<Vec<_>>();

  const MOD: u64 = 16777216;

  let n = nums.iter()
    .map(|n| std::iter::successors(Some(*n), |n| {
      let mut n = *n;
      n = ((n * 64) ^ n) % MOD;
      n = ((n / 32) ^ n) % MOD;
      n = ((n * 2048) ^ n) % MOD;
      Some(n)
    }).nth(2000).unwrap())
    .sum::<u64>();
              
  println!("{n}");
}