use std::io::BufRead;
use std::ops::BitAnd;

fn prio(c: char) -> u32 {
  match c {
    'a'..='z' => c as u32 - 'a' as u32 + 1,
    'A'..='Z' => c as u32 - 'A' as u32 + 27,
    _ => panic!("bad char {}", c),
  }
}

fn pres(s: String) -> u64 {
  s.chars().fold(0, |r, c| r | (1 << prio(c)))
} 

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines()
    .map(|r| r.unwrap())
    .take_while(|l| l.len() > 0);

  let mut r = 0;
  while let Some(c) = lines.by_ref().take(3)
      .map(pres).reduce(u64::bitand) {
    r += c.trailing_zeros();
  }

  println!("{}", r);
}