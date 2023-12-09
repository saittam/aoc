use std::io::BufRead;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut nums = lines.next().expect("line")
    .split_whitespace()
    .map(|w| w.parse::<u32>().expect("num"))
    .collect::<Vec<_>>();

  let l = nums.len();
  let mut seen = HashSet::new();
  let n = std::iter::from_fn(|| {
      let (p, n) = nums.iter().cloned().enumerate().rev()
        .max_by_key(|(_, n)| *n)
        .expect("max");
      nums[p] = 0;
      let (q, r) = (n / l as u32, n % l as u32);
      let idxi = (0..l).cycle().skip(p + 1).take(l);
      for i in idxi.clone() {
        nums[i] += q;
      }
      for i in idxi.take(r as usize) {
        nums[i] += 1;
      }
      Some(nums.clone())
    })
    .position(|v| !seen.insert(v))
    .expect("cycle");

  println!("{}", n + 1);
}