use std::io::BufRead;
use std::collections::HashMap;

// S(n) = 1 + \sum_{i = 9 + 7 * k .. n} S(n - i)
fn comp(m: &mut HashMap<u32, usize>, n: u32) -> usize {
  if let Some(v) = m.get(&n) {
    return *v;
  }
  let v = 1 + (9..n).step_by(7)
    .map(|r| comp(m, n - r)).sum::<usize>();
  m.insert(n, v);
  v
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let start = lines.next().unwrap().split(',')
    .map(|n| n.parse::<u32>().unwrap())
    .collect::<Vec<_>>();

  let mut m = HashMap::new();
  let s = start.iter()
    .map(|n| comp(&mut m, 256 + (9 - n))).sum::<usize>();
  println!("{}", s);
}