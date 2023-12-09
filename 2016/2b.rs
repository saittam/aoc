use std::io::BufRead;

enum Dir {
  Up = 0,
  Right = 1,
  Down = 2,
  Left = 3,
}
fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let dirs = lines
    .map(|l| l.chars()
      .map(|c| match c {
        'U' => Dir::Up,
        'R' => Dir::Right,
        'D' => Dir::Down,
        'L' => Dir::Left,
        _ => panic!("dir"),
      })
      .collect::<Vec<_>>())
    .collect::<Vec<_>>();

  const A: usize = 0xa;
  const B: usize = 0xb;
  const C: usize = 0xc;
  const D: usize = 0xd;
  const STEP: [[usize; 4]; 13] = [
    [ 1, 1, 3, 1 ],
    [ 2, 3, 6, 2 ],
    [ 1, 4, 7, 2 ],
    [ 4, 4, 8, 3 ],
    [ 5, 6, 5, 5 ],
    [ 2, 7, A, 5 ],
    [ 3, 8, B, 6 ],
    [ 4, 9, C, 7 ],
    [ 9, 9, 9, 8 ],
    [ 6, B, A, A ],
    [ 7, C, D, A ],
    [ 8, C, C, B ],
    [ B, D, D, D ],
  ];

  let code = dirs.iter()
    .scan(5, |start, seq| {
      *start = seq.iter().fold(
        *start, |pos, dir| STEP[pos - 1][*dir as usize]);
      Some(*start)
    })
    .fold(0, |n, d| n * 16 + d);

  println!("{:X}", code);
}