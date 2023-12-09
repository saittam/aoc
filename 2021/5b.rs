use std::io::BufRead;
use std::collections::HashMap;

fn xpair<I, E>(i: &mut I) -> (E, E)
where I: Iterator<Item = E> {
  (i.next().unwrap(), i.next().unwrap())
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let v = lines.take_while(|l| l.len() > 0)
    .map(|l| xpair(&mut l.split("->")
      .map(|s| xpair(&mut s.split(',')
        .map(|n| n.trim().parse::<i32>().unwrap())))))
    .collect::<Vec<_>>();
    
  let mut counts = HashMap::new();
  for ((x1, y1), (x2, y2)) in &v {
    let (dx, dy) = ((x2 - x1).signum(), (y2 - y1).signum());
    if dx == 0 || dy == 0 ||
       (x2 - x1).abs() == (y2 - y1).abs() {
      let (mut px, mut py) = (*x1, *y1);
      loop {
        *counts.entry((px, py)).or_insert(0) += 1;
        if (px, py) == (*x2, *y2) {
          break;
        }
        px += dx;
        py += dy;
      }
    }
  }
  
  let n = counts.values().filter(|c| **c > 1).count();
  println!("{}", n);
}