use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let n = lines
    .map(|l| l.split_whitespace()
              .filter_map(|w| w.parse::<i32>().ok())
              .fold(None, |mm, n| mm.or(Some((n, n)))
                .map(|(a, b)| (i32::min(a, n), i32::max(b, n))))
              .map(|(a, b)| (b - a))
              .expect("empty"))
    .sum::<i32>();
  
  println!("{}", n);
}