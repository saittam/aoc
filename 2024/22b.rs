use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let nums = lines
    .map(|l| l.parse::<i64>().expect("num"))
    .collect::<Vec<_>>();

  const MOD: i64 = 16777216;

  let counts = nums.iter().fold(
    HashMap::new(),
    |mut counts, n| {
      let cn = std::iter::successors(
        Some(*n), |n| {
          let mut n = *n;
          n = ((n * 64) ^ n) % MOD;
          n = ((n / 32) ^ n) % MOD;
          n = ((n * 2048) ^ n) % MOD;
          Some(n)
        })
        .take(2000 + 1)
        .map(|n| n % 10)
        .scan(0, |p, n| {
          let delta = n - *p;
          *p = n;
          Some((n, delta))
        })
        .skip(1)
        .scan(0u32, |seq, (n, delta)| {
          *seq = (*seq << 8) | (delta + 10) as u32;
          Some((*seq, n))
        })
        .skip(3)
        .fold(HashMap::new(), |mut counts, (seq, n)| {
          counts.entry(seq).or_insert(n);
          counts
        });
      for (seq, n) in cn {
        *counts.entry(seq).or_insert(0) += n;
      }
      counts
    });
    
  let n = counts.values().max().expect("max");
              
  println!("{n}");
}