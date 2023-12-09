use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut nums = lines
    .map(|l| l.split_whitespace()
              .find_map(|w| w.parse::<u64>().ok())
              .expect("num"));

  const N: u32 = 5_000_000;
  const MOD: u64 = 2147483647;
  let next = |mut v: u64, f: u64, mask: u64| loop {
    v = (v * f) % MOD;
    if v & mask == 0 {
      break v;
    }
  };
  let s = (nums.next().expect("a"), nums.next().expect("b"));
  let n = (0..N).scan(s, |(a, b), _| {
      *a = next(*a, 16807, 3);
      *b = next(*b, 48271, 7);
      Some((*a, *b))
    })
    .filter(|(a, b)| a & 0xffff == b & 0xffff)
    .count();
  
  println!("{}", n);
}