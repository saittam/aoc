use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let b = lines.next().unwrap()
    .split(|c: char| !(c.is_numeric() || c == '-'))
    .filter(|s| s.len() > 0)
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<_>>();
  
  // Assumes that target area is in negative y and wide
  // enough to have probe drop straight through with
  // suitable choice of x velocity.
  let t = (2.0 * b[1] as f64).sqrt() as i32;
  let tx = (t * (t + 1) / 2);
  assert!(b[0] <= tx);
  assert!(b[1] >= tx);
  assert!(b[3] < 0);
  assert!(b[2] <= b[3]);
  
  let vy = -b[2] - 1;
  let h = (vy * (vy + 1)) / 2;
  println!("{:?}", h);
}