use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let c = lines.take_while(|l| l.len() > 0).map(
    |l| l.split_whitespace()
         .skip(11)
         .filter(|s| [2, 3, 4, 7].contains(&s.len()))
         .count())
    .sum::<usize>();

  println!("{}", c);
}