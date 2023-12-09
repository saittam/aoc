use std::io::BufRead;

fn norm((x, y): (i32, i32)) -> i32 {
  if x.signum() == y.signum() {
    (x + y).abs()
  } else {
    i32::max(y.abs(), x.abs())
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let steps = lines.next().expect("input")
    .split(',')
    .map(|w| match w {
      "n" => (0, -1),
      "nw" => (-1, 0),
      "ne" => (1, -1),
      "s" => (0, 1),
      "se" => (1, 0),
      "sw" => (-1, 1),
      _ => panic!("step {}", w),
    })
    .collect::<Vec<_>>();

  let p = steps.iter()
    .fold((0i32, 0i32), |(x, y), (dx, dy)| (x + dx, y + dy));
  
  println!("{}", norm(p));
}