use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let (a, b) = lines
    .map(|l| {
      let mut ni = l.split_whitespace()
         .map(|w| w.parse::<u32>().expect("num"));
      (ni.next().expect("a"), ni.next().expect("b"))
    })
    .unzip::<u32, u32, Vec<_>, Vec<_>>();

  let n = a.iter()
    .map(|a| *a * b.iter().filter(|b| *a == **b).count() as u32)
    .sum::<u32>();

  println!("{n}");
}