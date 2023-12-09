use std::io::BufRead;

fn prio(c: char) -> u32 {
  match c {
    'a'..='z' => c as u32 - 'a' as u32 + 1,
    'A'..='Z' => c as u32 - 'A' as u32 + 27,
    _ => panic!("bad char {}", c),
  }
}

fn pres(s: &str) -> u64 {
  s.chars().fold(0, |r, c| r | (1 << prio(c)))
} 

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut r = 0;
  for l in lines.by_ref().take_while(|l| l.len() > 0) {
    let p = l.split_at(l.len() / 2);
    let c = pres(p.0) & pres(p.1);
    r += c.trailing_zeros();
  }

  println!("{}", r);
}