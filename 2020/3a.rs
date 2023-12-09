use std::io::BufRead;

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
  
  println!("{}", check(&m, (3, 1)));
}