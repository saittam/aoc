use std::io::BufRead;

fn hash(s: &str) -> u32 {
  s.chars().fold(0, |h, c| (h + c as u32) * 17 & 0xff)
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let n = lines.next().expect("input").split(',')
    .map(hash).sum::<u32>();
                     
  println!("{}", n);
}