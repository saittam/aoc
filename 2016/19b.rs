use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let n = lines.next().expect("input")
    .parse::<usize>().expect("number");

  let w = (4..=n)
    .fold(2, |w, k| {
      let victim = k / 2;
      if w + 1 < victim {
        w + 1
      } else if w + 2 < k {
        w + 2
      } else {
        0
      }
    });
  
  println!("{}", w + 1)
}