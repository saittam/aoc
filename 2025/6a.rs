use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let lines = lines.collect::<Vec<_>>();
  let mut wi = lines
    .iter()
    .map(|l| l.split_whitespace())
    .collect::<Vec<_>>();

  let problems = (0..)
    .map(|_| {
      wi.iter_mut()
        .flat_map(|i| i.next())
        .collect::<Vec<_>>()
    })
    .take_while(|p| !p.is_empty())
    .collect::<Vec<_>>();

  let n = problems
    .iter()
    .map(|p| {
      let mut wi = p.iter().rev();
      let op = match *wi.next().expect("op") {
        "+" => core::ops::Add::add,
        "*" => core::ops::Mul::mul,
        o => panic!("bad op {o}"),
      };

      let mut ni =
        wi.map(|w| w.parse::<u64>().expect("num"));
      let first = ni.next().expect("first");
      ni.fold(first, op)
    })
    .sum::<u64>();

  println!("{n}");
}
