use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let (mut a, mut b) = lines
    .map(|l| {
      let mut ni = l.split_whitespace()
         .map(|w| w.parse::<u32>().expect("num"));
      (ni.next().expect("a"), ni.next().expect("b"))
    })
    .unzip::<u32, u32, Vec<_>, Vec<_>>();

  a.sort();
  b.sort();

  let n = a.iter().zip(&b)
    .map(|(a, b)| a.abs_diff(*b))
    .sum::<u32>();

  println!("{n}");
}