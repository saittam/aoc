use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let rotations = lines
    .map(|l| {
      let mut chars = l.chars();
      let sig = match chars.next().expect("first") {
        'L' => -1,
        'R' => 1,
        d => panic!("Bad direction {d}"),
      };
      let dist =
        chars.as_str().parse::<i32>().expect("distance");
      sig * dist
    })
    .collect::<Vec<_>>();

  let n = rotations
    .iter()
    .scan(50, |pos, rot| {
      *pos = (*pos + rot).rem_euclid(100);
      Some(*pos)
    })
    .filter(|p| *p == 0)
    .count();

  println!("{n}");
}
