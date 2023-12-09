use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  const DIGITS: [char; 5] = [ '=', '-', '0', '1', '2' ];
  let mut s = lines
    .map(|l| l.chars().fold(
      0isize, 
      |a, c| {
        let d = DIGITS.iter()
          .position(|d| *d == c)
          .expect("digit");
        a * 5 + d as isize - 2
      }))
    .sum::<isize>();

  let mut enc = Vec::new();
  while s > 0 {
    s = s + 2;
    enc.push(DIGITS[s as usize % 5]);
    s = s / 5;
  }
  
  println!("{}", enc.iter().rev().collect::<String>());
}