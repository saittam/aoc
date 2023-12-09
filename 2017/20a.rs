use std::io::BufRead;

type V3 = (i64, i64, i64);

fn norm((x, y, z): &V3) -> i64 {
  x.abs() + y.abs() + z.abs()
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let particles = lines
    .map(|l| {
      let mut numi = l
        .split(|c: char| c != '-' && !c.is_numeric())
        .filter_map(|p| p.parse::<i64>().ok());

      let mut triple = || (numi.next().expect("1"),
                           numi.next().expect("2"),
                           numi.next().expect("3"));

      (triple(), triple(), triple())
    })
    .collect::<Vec<_>>();

  let (i, _) = particles.iter()
    .enumerate()
    .min_by_key(|(_, (_, _, a))| norm(a))
    .expect("min");

  println!("{}", i);
}