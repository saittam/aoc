use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let batteries = lines
    .map(|l| {
      l.chars()
        .map(|c| c.to_digit(10).expect("digit") as u64)
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let n = batteries
    .iter()
    .map(|v| {
      (0..12)
        .rev()
        .fold((0, 0), |(j, p), k| {
          let (pk, jk) = v
            .iter()
            .enumerate()
            .skip(p)
            .rev()
            .skip(k)
            .max_by_key(|(_, v)| **v)
            .expect("max");
          (j + jk * 10u64.pow(k as u32), pk + 1)
        })
        .0
    })
    .sum::<u64>();

  println!("{n}");
}
