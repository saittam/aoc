use std::io::BufRead;

fn check(result: u64, head: u64, tail: &[u64]) -> bool {
  match tail.first() {
    None => result == head,
    Some(v) => result >= head &&
      (check(result, head + v, &tail[1..]) ||
       check(result, head * v, &tail[1..])),
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let eqs = lines
    .map(|l| {
      let mut ni = l.split(|c: char| !c.is_digit(10))
        .filter(|w| !w.is_empty())
        .map(|w| w.parse::<u64>().expect("num"));
      (ni.next().expect("result"), ni.collect::<Vec<_>>())
    })
    .collect::<Vec<_>>();

  let n = eqs.iter()
    .filter(|(r, o)| check(*r, o[0], &o[1..]))
    .map(|(r, _)| r)
    .sum::<u64>();
  
  println!("{}", n);
}