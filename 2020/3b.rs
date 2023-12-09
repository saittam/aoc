use std::io::BufRead;
use std::ops::Mul;

#[derive(PartialEq, Eq)]
enum Square {
  Open,
  Tree,
}

fn check(m: &Vec<Vec<Square>>,
         (dx, dy): (usize, usize)) -> usize {
  let mut ntrees = 0;
  let mut p = 0;
  for d in m.iter().step_by(dy) {
    if d[p] == Square::Tree {
      ntrees += 1;
    }
    p = (p + dx ) % d.len();
  }
  
  ntrees
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut m = Vec::new();
  loop {  
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }
    
    let p = buf.trim().chars().map(
      |c| match c {
        '.' => Square::Open,
        '#' => Square::Tree,
        _ => panic!("Bad square: {}", c),
      }).collect::<Vec<Square>>();
    m.push(p);
  }
  
  let r = [(1,1), (3,1), (5,1), (7,1), (1,2)]
    .iter()
    .map(|s| check(&m, *s))
    .fold(1, usize::mul);
  println!("{}", r);
}