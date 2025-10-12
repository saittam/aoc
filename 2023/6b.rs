use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut  nums = lines.map(|l| l
    .replace(|c: char| !c.is_digit(10), "")
    .parse::<i64>()
    .expect("num"));
  let time = nums.next().expect("time");
  let dist = nums.next().expect("dist");

  let n = (0..=time)
    .map(|b| (time - b) * b)
    .filter(|bd| *bd > dist)
    .count();
 
  println!("{}", n);
}