use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let line = lines.next().expect("input");
  let dirs = line
    .split(',')
    .map(|w| {
      let mut ci = w.trim().chars();
      let d = match ci.next().expect("dir") {
        'L' => 3,
        'R' => 1,
        _ => panic!("dir"),
      };
      (d, ci.as_str().parse::<i64>().expect("steps"))
    })
    .collect::<Vec<_>>();

  const DELTA: [(i64, i64); 4] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
  ];
  let ((x, y), _) = dirs.iter()
    .fold(((0, 0), 0), |((x, y), d), (t, s)| {
      let dn = (d + t) % 4;
      let (dx, dy) = DELTA[dn];
      ((x + s * dx, y + s * dy), dn)
    });
  
  println!("{}", x.abs() + y.abs());
}