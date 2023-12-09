use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let line = lines.next().expect("input");
  let mut nums = line.split(|c: char| !c.is_digit(10))
                     .filter_map(|w| w.parse::<u64>().ok());
  let row = nums.next().expect("row");
  let col = nums.next().expect("col");

  let r = row + col - 1;
  let mut n = r * (r + 1) / 2 - row;

  const START: u64 = 20151125;
  const MOD: u64 = 33554393;
  const FACTOR: u64 = 252533;
  let mut f = 1;
  let mut df = FACTOR;
  while n > 0 {
    f = if n % 2 == 1 { f * df } else { f } % MOD;
    df = df * df % MOD;
    n = n / 2;
  }
  
  println!("{}", (START * f) % MOD);
}