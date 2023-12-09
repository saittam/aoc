use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let n = lines.next().expect("input")
    .parse::<usize>().expect("num");

  let mut buf = vec![0];
  let mut p = 0;
  for i in 1..=2017 {
    p = (p + n) % i + 1;
    buf.insert(p, i);
  }

  println!("{}", buf[(p + 1) % buf.len()]);
}