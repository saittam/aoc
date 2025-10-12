use std::io::Read;

fn main() {
  let stdin = std::io::stdin();
  let mut input = String::new();
  stdin.lock().read_to_string(&mut input).expect("input");

  let nums = input.chars().map(
    |c| c.to_digit(10).expect("digit"))
    .collect::<Vec<_>>();

  let used = nums.iter().step_by(2).sum::<u32>();
  let blocks = nums.iter()
    .enumerate()
    .flat_map(
      |(i, n)| 
      std::iter::repeat((i % 2 == 0).then_some(i / 2))
      .take(*n as usize));

  let mut rev = blocks.clone().rev().filter_map(|b| b);
  let n = blocks
    .map(|b| b.unwrap_or_else(|| rev.next().expect("rev")))
    .take(used as usize)
    .enumerate()
    .map(|(i, b)| i * b)
    .sum::<usize>();
    
  println!("{n}");
}