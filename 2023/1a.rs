use std::io::BufRead;

fn dig<I: Iterator<Item=char>>(mut i: I) -> u32 {
    i.find_map(|c| c.to_digit(10)).expect("digit")
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let n = lines
    .map(|l| 10 * dig(l.chars()) + dig(l.chars().rev()))
    .sum::<u32>();
  
  println!("{}", n);
}