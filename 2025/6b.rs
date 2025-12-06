use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let lines = lines.collect::<Vec<_>>();
  let mut cis =
    lines.iter().map(|l| l.chars()).collect::<Vec<_>>();

  type Op = fn(u64, u64) -> u64;
  let mut coli = std::iter::from_fn(|| {
    Some(
      cis
        .iter_mut()
        .map(|ci| ci.next().unwrap_or(' '))
        .fold((None, None), |(n, op), c| match c {
          '0'..='9' => {
            let d = c.to_digit(10).unwrap() as u64;
            let n = Some(n.unwrap_or(0) * 10 + d);
            (n, op)
          }
          '+' => (n, Some(core::ops::Add::add as Op)),
          '*' => (n, Some(core::ops::Mul::mul as Op)),
          c if c.is_whitespace() => (n, op),
          _ => panic!("bad input char {c}"),
        }),
    )
  });

  let n = std::iter::from_fn(|| {
    Some(
      coli.by_ref().take_while(|(n, _)| n.is_some()).fold(
        (Vec::new(), None),
        |(mut v, cop), (n, op)| {
          v.push(n.unwrap());
          let cop = cop.or(op);
          (v, cop)
        },
      ),
    )
  })
  .take_while(|(v, _)| !v.is_empty())
  .map(|(v, op)| {
    let mut ni = v.iter().copied();
    let first = ni.next().expect("first");
    ni.fold(first, op.expect("op"))
  })
  .sum::<u64>();

  println!("{n}");
}
