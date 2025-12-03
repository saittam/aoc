use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let batteries = lines
    .map(|l| {
      l.chars()
        .map(|c| c.to_digit(10).expect("digit"))
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let n = batteries
    .iter()
    .map(|v| {
      let (p1, v1) = v
        .iter()
        .enumerate()
        .rev()
        .skip(1)
        .max_by_key(|(_, v)| **v)
        .expect("max1");
      let v2 = v.iter().skip(p1 + 1).max().expect("max2");
      v1 * 10 + v2
    })
    .sum::<u32>();

  println!("{n}");
}
