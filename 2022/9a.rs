use std::io::BufRead;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let (mut hx, mut hy) = (0, 0);
  let (mut tx, mut ty) = (0, 0);
  let mut vis = HashSet::new();
  for l in lines.take_while(|l| l.len() > 0) {
    let mut w = l.split(' ');
    let d = w.next().expect("dir");
    let n = w.next().expect("dist")
      .parse::<isize>().expect("num");
    let (mx, my) = match d {
      "U" => (0, 1),
      "D" => (0, -1),
      "R" => (1, 0),
      "L" => (-1, 0),
      _ => panic!("dir {}", d),
    };
    for _ in 0..n {
      hx += mx;
      hy += my;
      let (dx, dy): (isize, isize) = (hx - tx, hy - ty);
      if dx.abs() > 1 || dy.abs() > 1 {
        tx += dx.signum();
        ty += dy.signum();
      }
      vis.insert((tx, ty));
    }
  }

  println!("{}", vis.len());
}