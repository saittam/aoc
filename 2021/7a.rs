use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut p = lines.next().unwrap().split(',')
    .map(|n| n.parse::<i32>().unwrap())
    .collect::<Vec<_>>();
    
  p.sort();
  let m = p[p.len() / 2];
  let c = p.iter().map(|p| (m - p).abs()).sum::<i32>();
  println!("{}", c);
}