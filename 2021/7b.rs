use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut p = lines.next().unwrap().split(',')
    .map(|n| n.parse::<i32>().unwrap())
    .collect::<Vec<_>>();
    
  let min = *p.iter().min().unwrap();
  let max = *p.iter().max().unwrap();
  let c = (min..=max)
    .map(|a| p.iter().map(|p| {
      let d = (a - p).abs();
      (d * (d + 1)) / 2
    }).sum::<i32>())
    .min().unwrap();
  println!("{}", c);
}