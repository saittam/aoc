use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let npairs = ["ab", "cd", "pq", "xy"];
  let c = lines.take_while(|s| s.len() > 0)
    .filter(|s|
      s.chars().filter(|c| "aeiou".contains(*c)).count() >= 3 &&
      s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b) &&
      !npairs.iter().any(|e| s.contains(e)))
    .count();
      
  println!("{}", c);
}