use std::io::BufRead;

const DIR: [(isize, isize); 4] = [
  (1, 0), (0, 1), (-1, 0), (0, -1),
];

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let map = lines
    .map(|l| {
      let mut wi = l.split_whitespace();
      let dw = wi.next().expect("dir");
      let dir = ["R", "D", "L", "U"].iter()
        .position(|d| *d == dw)
        .expect("bad dir");
      let len = wi.next().expect("len")
        .parse::<isize>().expect("num");
      let color = wi.next().expect("color");
      (dir, len, color.to_owned())
    })
    .collect::<Vec<_>>();

  let (a, c, end) = map.iter().fold(
    (0, 0, (0, 0)),
    |(a, c, (x, y)), (dir, len, _)| {
      let (dx, dy) = DIR[*dir];
      let (lx, ly) = (dx * len, dy * len);
      let (nx, ny) = (x + lx, y + ly);
      (a + (ny - y) * x, c + lx.abs() + ly.abs(), (nx, ny))
    });
  assert_eq!(end, (0, 0));

  println!("{}", a + c / 2 + 1);
}