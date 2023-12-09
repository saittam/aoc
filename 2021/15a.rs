use std::io::BufRead;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

fn get(m: &Vec<Vec<usize>>, p: (isize, isize)) -> Option<usize> {
  m.get(p.1 as usize).and_then(|r| r.get(p.0 as usize)).cloned()
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut m = lines.take_while(|l| l.len() > 0).map(
    |l| l.chars()
         .map(|c| c.to_digit(10).unwrap() as usize)
         .collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let mut seen = HashSet::new();
  let mut q = BinaryHeap::new();
  seen.insert((0, 0));
  q.push(Reverse((0, (0, 0))));
  while let Some(Reverse((c, (x, y)))) = q.pop() {
    if y as usize == m.len() - 1 &&
       x as usize == m.last().unwrap().len() - 1 {
      println!("{}", c);
      break;
    }
    
    let neighbors = [
      (x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)];
    for n in &neighbors {
      if !seen.insert(*n) {
        continue;
      }
      if let Some(r) = get(&m, *n) {
        q.push(Reverse((c + r, *n)));
      }
    }
  }
}