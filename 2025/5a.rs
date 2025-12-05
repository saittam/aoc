use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let ranges = lines
    .by_ref()
    .take_while(|l| l.len() > 0)
    .map(|l| {
      let mut ni = l
        .splitn(2, '-')
        .map(|n| n.parse::<u64>().expect("num"));
      ni.next().expect("lower")..=ni.next().expect("upper")
    })
    .collect::<Vec<_>>();

  let ingredients = lines
    .map(|n| n.parse::<u64>().expect("num"))
    .collect::<Vec<_>>();

  let n = ingredients
    .iter()
    .filter(|i| ranges.iter().any(|r| r.contains(i)))
    .count();

  println!("{n}");
}
