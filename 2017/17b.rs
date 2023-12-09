use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let n = lines.next().expect("input")
    .parse::<usize>().expect("num");

  let mut p1 = 0;
  let mut p = 0;
  for i in 1..=50000000 {
    p = (p + n) % i + 1;
    if p == 1 {
      p1 = i;
    }
  }

  println!("{}", p1);
}