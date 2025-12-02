use std::collections::HashSet;
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
        .flat_map(|len| {
          // compute all possible splits into groups
          (1..=(len / 2)).filter_map(move |glen| {
            ((len % glen) == 0).then(|| (glen, len / glen))
          })
        })
        .flat_map(move |(glen, rep)| {
          // enumerate the invalid ID patterns
          let gdiv = 10u64.pow(glen);
          let hidiv = gdiv.pow(rep - 1);
          let patlb = u64::max(lb / hidiv, gdiv / 10);
          let patub = u64::min(ub / hidiv, gdiv - 1);
          (patlb..=patub)
            .map(move |gp| {
              // concat groups to form the ID
              (0..rep).fold(0, |p, _| p * gdiv + gp)
            })
            .filter(move |p| lb <= *p && *p <= ub)
        })
        // HashSet to deduplicate. Could iterate all
        // classes of patterns in parallel to dedup
        // without needing a set.
        .collect::<HashSet<_>>()
        .iter()
        .sum::<u64>()
    })
    .sum::<u64>();

  println!("{n}");
}
