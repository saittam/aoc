use std::io::BufRead;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Dir {
  E,
  SE,
  SW,
  W,
  NW,
  NE,
}
  
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
    const MDir: [(isize, isize); 6] = [
      (1, 0),
      (1, 1),
      (0, 1),
      (-1, 0),
      (-1, -1),
      (0, -1),
    ];
    let p = dv.iter()
              .map(|d| MDir[*d as usize])
              .fold((0, 0), |p, m| (p.0 + m.0, p.1 + m.1));
    if !black.remove(&p) {
      black.insert(p);
    }
  }
  
  println!("{}", black.len());
}