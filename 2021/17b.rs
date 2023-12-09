use std::io::BufRead;

fn launch((mut x, mut y): (i32, i32),
          (mut vx, mut vy): (i32, i32),
          ((lx, ly), (ux, uy)): ((i32, i32), (i32, i32)))
  -> bool {
  loop {
    if lx <= x && x <= ux && ly <= y && y <= uy {
      return true;
    }
    if x > ux || y < ly {
      return false;
    }
    x += vx;
    y += vy;
    vx = if vx == 0 { 0 } else { vx - 1 };
    vy -= 1;
  }
}
  
fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let b = lines.next().unwrap()
    .split(|c: char| !(c.is_numeric() || c == '-'))
    .filter(|s| s.len() > 0)
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<_>>();
  
  // Assumes that target area is in negative y and 
  // positive x. 
  assert!(b[0] >= 0);
  assert!(b[1] >= b[0]);
  assert!(b[3] < 0);
  assert!(b[2] <= b[3]);
  
  let ltx = b[0];
  let utx = b[1];
  let lty = b[2];
  let uty = b[3];
  let target = ((ltx, lty), (utx, uty));
  
  let mut hits = 0;
  for vx in 0..=utx {
    for vy in lty..=-lty {
      hits += launch((0, 0), (vx, vy), target) as usize;
    }
  }
  
  println!("{:?}", hits);
}