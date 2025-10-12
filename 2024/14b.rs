use std::io::BufRead;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let robots = lines
    .map(|l| {
      let mut li = l
        .split(|c: char| !c.is_digit(10) && c != '-')
        .filter_map(|w| w.parse::<i32>().ok());
      ((li.next().expect("x"), li.next().expect("y")),
       (li.next().expect("vx"), li.next().expect("vy")))
    })
    .collect::<Vec<_>>();

  const W: i32 = 101;
  const H: i32 = 103;

  // find first arrangement where more than half of
  // the robots have a neighbor.
  let n = (0..).find(
    |n| {
      let pos = robots.iter()
        .map(|(p, (vx, vy))| (p, ((vx + W) % W,
                                  (vy + H) % H)))
        .map(|((x, y), (vx, vy))|
             ((x + n * vx) % W, (y + n * vy) % H))
        .collect::<HashSet<_>>();
      let np = [(-1, 0), (0, -1), (1, 0), (0, 1)];
      let s = pos.iter()
        .filter(|(x, y)| np.iter().any(|(nx, ny)|
                pos.contains(&(x + nx, y + ny))))
        .count();
      s > robots.len() / 2
    })
    .unwrap();
  
  println!("{n}");
}