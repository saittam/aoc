use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let line = lines.next().expect("input");
  let n = line.chars().cycle().skip(1).zip(line.chars())
    .filter(|(a, b)| a == b)
    .map(|(a, _)| a.to_digit(10).expect("digit"))
    .sum::<u32>();
  
  println!("{}", n);
}