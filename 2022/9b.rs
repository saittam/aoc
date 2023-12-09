use std::io::BufRead;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  const N: usize = 9;
  let (mut hx, mut hy) = (0, 0);
  let mut kn = [(0, 0); N];
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
      let (mut px, mut py) = (hx, hy);
      for (tx, ty) in kn.iter_mut() {
        let (dx, dy): (isize, isize) = (px - *tx, py - *ty);
        if dx.abs() > 1 || dy.abs() > 1 {
          *tx += dx.signum();
          *ty += dy.signum();
        }
        px = *tx;
        py = *ty;
      }
      vis.insert(kn[N - 1]);
    }
  }

  println!("{}", vis.len());
}