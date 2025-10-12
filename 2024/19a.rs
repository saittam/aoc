use std::io::BufRead;

fn make(towels: &[String], pattern: &str) -> bool {
  pattern.is_empty() ||
  towels.iter().any(
    |p| pattern
    .strip_prefix(p)
    .and_then(|s| Some(make(towels, s)))
    .unwrap_or(false))
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let towels = lines.next().expect("towels")
    .split(',')
    .map(|w| w.trim().to_owned())
    .collect::<Vec<_>>();
  
  let patterns = lines.skip(1).collect::<Vec<_>>();

  let n = patterns.iter()
    .filter(|p| make(&towels, p))
    .count();

  println!("{n}");
}