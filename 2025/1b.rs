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
      let p = *pos + rot;
      let zeros = p.abs() as usize / 100
        + (*pos > 0 && p <= 0) as usize;
      *pos = p.rem_euclid(100);
      Some(zeros)
    })
    .sum::<usize>();

  println!("{n}");
}
