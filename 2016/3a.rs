use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let tris = lines
    .map(|l| {
      let mut nums = l
        .split_whitespace()
        .map(|w| w.parse::<u32>().expect("num"))
        .collect::<Vec<_>>();
      nums.sort();
      nums
    })
    .collect::<Vec<_>>();

  let n = tris.iter().filter(|t| t[0] + t[1] > t[2]).count();
  println!("{}", n);
}