use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let patterns = std::iter::repeat_with(
    || lines.by_ref()
      .take_while(|l| !l.is_empty())
      .enumerate()
      .fold(Vec::new(),
            |p, (i, l)| p.into_iter()
            .chain(std::iter::repeat(0))
            .zip(l.chars().map(|c| c == '#'))
            .map(|(a, b)| a | ((b as u32) << i))
            .collect::<Vec<_>>()))
    .take_while(|p| !p.is_empty())
    .collect::<Vec<_>>();

  let n = patterns.iter().enumerate().flat_map(
    |(i, p)| patterns.iter().skip(i + 1).map(
      move |q| (p, q)))
    .filter(
      |(p, q)| p.iter().zip(*q).all(|(a, b)| (a & b) == 0))
    .count();
              
  println!("{n}");
}