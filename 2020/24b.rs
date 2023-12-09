use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Dir {
  E,
  SE,
  SW,
  W,
  NW,
  NE,
}

const MDir: [(isize, isize); 6] = [
  (1, 0),
  (1, 1),
  (0, 1),
  (-1, 0),
  (-1, -1),
  (0, -1),
];
  
fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut v = Vec::new();
  for line in lines.by_ref().take_while(|l| l.len() > 0) {
    let mut dv = Vec::new();
    let mut ci = line.chars();
    while let Some(c) = ci.next() {
      let d = match c {
        'e' => Dir::E,
        's' => match ci.next().unwrap() {
          'e' => Dir::SE,
          'w' => Dir::SW,
          c => panic!("s{}", c),
        }
        'w' => Dir::W,
        'n' => match ci.next().unwrap() {
          'w' => Dir::NW,
          'e' => Dir::NE,
          c => panic!("s{}", c),
        },
        _ => panic!("{}", c),
      };
      dv.push(d);
    }
    v.push(dv);
  }
  
  let mut black = HashSet::new();
  for dv in &v {
    let p = dv.iter()
              .map(|d| MDir[*d as usize])
              .fold((0, 0), |p, m| (p.0 + m.0, p.1 + m.1));
    if !black.remove(&p) {
      black.insert(p);
    }
  }
  
  for _n in 0..100 {
    let mut count = HashMap::new();
    for p in &black {
      count.entry(*p).or_insert(0);
      for m in &MDir {
        *count.entry((p.0 + m.0, p.1 + m.1)).or_insert(0) += 1;
      }
    }
    
    for (p, n) in count {
      if black.contains(&p) {
        if n == 0 || n > 2 {
          black.remove(&p);
        }
      } else {
        if n == 2 {
          black.insert(p.clone());
        }
      }
    }
  }
  
  println!("{}", black.len());
}