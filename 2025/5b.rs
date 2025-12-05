use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut bounds = lines
    .by_ref()
    .take_while(|l| l.len() > 0)
    .map(|l| {
      let mut ni = l
        .splitn(2, '-')
        .map(|n| n.parse::<u64>().expect("num") as i64);
      ni.next().expect("lower")..=ni.next().expect("upper")
    })
    .flat_map(|r| [-r.start(), r.end() + 1])
    .collect::<Vec<_>>();

  lines.for_each(drop);

  bounds.sort_by_key(|b| b.abs());

  let n = bounds
    .iter()
    .fold((0, 0), |(np, cp), b| {
      let c = cp + b.signum();
      let n = np + if cp == 0 || c == 0 { *b } else { 0 };
      (n, c)
    })
    .0;

  println!("{n}");
}
