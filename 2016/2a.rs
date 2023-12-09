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

  const STEP: [[usize; 4]; 9] = [
    [ 1, 2, 4, 1 ],
    [ 2, 3, 5, 1 ],
    [ 3, 3, 6, 2 ],
    [ 1, 5, 7, 4 ],
    [ 2, 6, 8, 4 ],
    [ 3, 6, 9, 5 ],
    [ 4, 8, 7, 7 ],
    [ 5, 9, 8, 7 ],
    [ 6, 9, 9, 8 ],
  ];

  let code = dirs.iter()
    .scan(5, |start, seq| {
      *start = seq.iter().fold(
        *start, |pos, dir| STEP[pos - 1][*dir as usize]);
      Some(*start)
    })
    .fold(0, |n, d| n * 10 + d);

  println!("{}", code);
}