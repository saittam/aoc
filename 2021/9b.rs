use std::io::BufRead;
use std::collections::HashSet;
use std::collections::VecDeque;

fn get(m: &Vec<Vec<u8>>, p: (isize, isize)) -> Option<u8> {
  m.get(p.1 as usize).and_then(|r| r.get(p.0 as usize).cloned())
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let m = lines.take_while(|l| l.len() > 0).map(
    |l| l.chars()
         .map(|c| c.to_digit(10).unwrap() as u8)
         .collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let mut seen = HashSet::new();
  seen.insert((0, 0));
  let mut aq = VecDeque::new();
  let mut bq = VecDeque::new();
  let h = get(&m, (0, 0)).unwrap();
  if h == 9 { &mut bq } else { &mut aq }.push_back((0, 0));
  let mut a = 0;
  let mut areas = Vec::new();
  loop {
    let (x, y) = match aq.pop_front() {
      Some(p) => {
        a += 1;
        p
      }
      None => {
        areas.push(a);
        a = 0;
        match bq.pop_front() {
          Some(p) => p,
          None => break,
        }
      }
    };
    
    let neighbors =
      [ (x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1) ];
    for n in &neighbors {
      if seen.contains(n) {
        continue;
      }
      if let Some(h) = get(&m, *n) {
        seen.insert(*n);
        if h == 9 { &mut bq } else { &mut aq }.push_back(*n);
      }
    }
  }

  areas.sort();
  let s: usize = areas.iter().rev().take(3).product();

  println!("{}", s);
}