use std::io::BufRead;
use std::collections::HashMap;

fn count<'a>(towels: &[String],
             pattern: &'a str,
             cache: &mut HashMap<&'a str, usize>) -> usize {
  if pattern.is_empty() {
    return 1;
  }
  
  if let Some(n) = cache.get(pattern) {
    return *n;
  }
  
  let n = towels.iter()
    .map(
      |p| pattern
      .strip_prefix(p)
      .and_then(|s| Some(count(towels, s, cache)))
      .unwrap_or(0))
    .sum::<usize>();
  cache.insert(pattern, n);
  n
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let towels = lines.next().expect("towels")
    .split(',')
    .map(|w| w.trim().to_owned())
    .collect::<Vec<_>>();
  
  let patterns = lines.skip(1).collect::<Vec<_>>();

  let mut cache = HashMap::new();
  let n = patterns.iter()
    .map(|p| count(&towels, p, &mut cache))
    .sum::<usize>();

  println!("{n}");
}