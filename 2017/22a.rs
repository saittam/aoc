use std::io::BufRead;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut xdim = (i32::MAX, i32::MIN);
  let mut ydim = (i32::MAX, i32::MIN);
  let mut inf = lines
    .enumerate()
    .inspect(|(y, _)| ydim = (ydim.0.min(*y as i32),
                              ydim.1.max(*y as i32)))
    .fold(HashSet::new(), |mut s, (y, l)| {
      s.extend(
        l.chars()
         .enumerate()
         .inspect(|(x, _)| xdim = (xdim.0.min(*x as i32),
                                   xdim.1.max(*x as i32)))
         .filter(|(_, c)| *c == '#')
         .map(|(x, _)| (x as i32, y as i32)));
      s
    });

  const STEP: [(i32, i32); 4] = [
    (0, -1), (1, 0), (0, 1), (-1, 0)
  ];
  
  let mut pos = (xdim.0 + (xdim.1 - xdim.0) / 2,
                 ydim.0 + (ydim.1 - ydim.0) / 2);
  let mut dir = 0;
  let mut count = 0;
  for _ in 0..10000 {
    if inf.remove(&pos) {
      dir = (dir + 1) % 4;
    } else {
      inf.insert(pos);
      count += 1;
      dir = (dir + 3) % 4;
    }
    let step = STEP[dir];
    pos = (pos.0 + step.0, pos.1 + step.1);
  }
      
  println!("{}", count);
}