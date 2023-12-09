use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut nums = lines
    .map(|l| l.split_whitespace()
              .find_map(|w| w.parse::<u64>().ok())
              .expect("num"));

  const N: u32 = 40_000_000;
  const MOD: u64 = 2147483647;
  let s = (nums.next().expect("a"), nums.next().expect("b"));
  let n = (0..N).scan(s, |(a, b), _| {
      *a = (*a * 16807) % MOD;
      *b = (*b * 48271) % MOD;
      Some((*a, *b))
    })
    .filter(|(a, b)| a & 0xffff == b & 0xffff)
    .count();
  
  println!("{}", n);
}