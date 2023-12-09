use std::io::BufRead;
use std::collections::HashSet;

fn wins(b: &Vec<Vec<u32>>, d: &HashSet<u32>) -> bool {
  b.iter().any(
    |r| r.iter().all(
      |n| d.contains(n))) ||
  (0..b.len()).any(
    |c| b.iter().all(
      |r| d.contains(r.get(c).unwrap())))
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let seq = lines.next().unwrap()
    .split(',')
    .map(|s| s.parse::<u32>().unwrap())
    .collect::<Vec<_>>();
    
  lines.next();
  
  let mut boards = Vec::new();
  loop {
    let b = lines.by_ref()
      .take_while(|l| l.len() > 0)
      .map(
        |l| l.trim().split_whitespace()
          .map(|s| s.parse::<u32>().unwrap())
          .collect::<Vec<_>>())
      .collect::<Vec<_>>();
      
    if b.len() == 0 {
      break;
    }
    boards.push(b);
  }
  
  let mut drawn = HashSet::new();
  for n in seq {
    drawn.insert(n);
    
    let (win, open): (Vec<_>, Vec<_>) =
      boards.drain(0..).partition(|b| wins(b, &drawn));
    if open.len() == 0 {
      if let Some(b) = win.first() {
        let s = b.iter().flatten()
          .filter(|n| !drawn.contains(n))
          .sum::<u32>();
        println!("{}", n * s);
      }
      break;
    } else {
      boards = open;
    }
  }
}