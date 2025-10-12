use std::io::BufRead;

const DIR: [(isize, isize); 4] = [
  (1, 0), (0, 1), (-1, 0), (0, -1),
];

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let map = lines
    .map(|l| {
      let mut hexi = 
        l.split('#').skip(1).next().expect("hex").chars();
      let len = hexi.by_ref().take(5)
        .map(|c| c.to_digit(16).expect("len digit") as isize)
        .fold(0, |n, d| n * 16 + d);
      let dir = hexi.next().expect("dir")
        .to_digit(4).expect("dir digit") as usize;
      (dir, len)
    })
    .collect::<Vec<_>>();

  let (a, c, end) = map.iter().fold(
    (0, 0, (0, 0)),
    |(a, c, (x, y)), (dir, len)| {
      let (dx, dy) = DIR[*dir];
      let (lx, ly) = (dx * len, dy * len);
      let (nx, ny) = (x + lx, y + ly);
      (a + (ny - y) * x, c + lx.abs() + ly.abs(), (nx, ny))
    });
  assert_eq!(end, (0, 0));

  println!("{}", a + c / 2 + 1);
}