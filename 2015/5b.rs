use std::io::BufRead;

fn pairs(s: &str) -> bool {
  let v = s.as_bytes();
  (0..(v.len() - 3)).any(
    |i| ((i + 2)..(v.len() - 1)).any(
      |j| v[i] == v[j] && v[i + 1] == v[j + 1]))
}

fn triple(s: &str) -> bool {
  s.chars().zip(s.chars().skip(2)).any(|(a, b)| a == b)
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let c = lines.take_while(|s| s.len() > 0)
    .filter(|s| pairs(s) && triple(s))
    .count();
      
  println!("{}", c);
}