use std::io::Read;

fn main() {
  let mut stdin = std::io::stdin();
  let mut line = String::new();
  stdin.read_to_string(&mut line).expect("line");

  let ranges = line
    .split(',')
    .map(|r| {
      let mut ni = r
        .splitn(2, '-')
        .map(|n| n.trim().parse::<u64>().expect("num"));
      (
        ni.next().expect("lower bound"),
        ni.next().expect("upper bound"),
      )
    })
    .collect::<Vec<_>>();

  let n = ranges
    .iter()
    .filter(|(lb, ub)| lb <= ub)
    .cloned()
    .map(|(lb, ub)| {
      // number of digits for bounds
      let lenlb = lb.ilog10() + 1;
      let lenub = ub.ilog10() + 1;

      // for all relevant digit counts
      (lenlb..=lenub)
        .filter(|len| len % 2 == 0)
        .map(|len| len / 2)
        .flat_map(move |glen| {
          // enumerate the invalid ID patterns
          let div = 10u64.pow(glen);
          let patlb = u64::max(lb / div, div / 10);
          let patub = u64::min(ub / div, div - 1);
          (patlb..=patub)
            .map(move |gp| gp * div + gp)
            .filter(move |p| lb <= *p && *p <= ub)
        })
        .sum::<u64>()
    })
    .sum::<u64>();

  println!("{n}");
}
