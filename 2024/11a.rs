use std::io::Read;

fn count(v: u64, n: usize) -> usize {
  if n == 0 {
    1
  } else if v == 0 {
    count(1, n - 1)
  } else {
    let d =
      std::iter::successors(Some(1), |d| Some(d * 10))
      .find(|d| d * d > v)
      .expect("len");
    if d * d <= v * 10 {
      count(v / d, n - 1) + count(v % d, n - 1)
    } else {
      count(v * 2024, n - 1)
    }
  }
}
      
fn main() {
  let stdin = std::io::stdin();
  let mut input = String::new();
  stdin.lock().read_to_string(&mut input).expect("input");

  let nums = input.split_whitespace()
    .map(|w| w.parse::<u64>().expect("num"))
    .collect::<Vec<_>>();

  let n = nums.iter()
    .map(|v| count(*v, 25))
    .sum::<usize>();
    
  println!("{n}");
}