use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut s = Vec::new();
  for l in lines {
    let n = l
      .split(|c: char| !c.is_digit(10) && c != '-')
      .filter(|w| w.len() > 0)
      .map(|n| n.parse::<isize>().expect("num"))
      .collect::<Vec<_>>();
    let r = (n[2] - n[0]).abs() + (n[3] - n[1]).abs();
    s.push(((n[0], n[1]), r));
  }

  const SIZE: isize = 4000000;

  let mut bounds = s.iter()
    .flat_map(|((x, y), r)| {
      let d = (x - y - r, x - y + r);
      [(x + y - r, -1, d), (x + y + r + 1, 1, d)]
    })
    .collect::<Vec<_>>();
  bounds.sort();
  let lines = s.iter().flat_map(
    |((x, y), r)| [(x - y + r + 1), (x - y - r - 1)]);
  for off in lines {
    let p = bounds.iter()
      .filter(|(_, _, (l, u))| *l <= off && off <= *u)
      .scan(0, |cov, (bound, e, _)| {
        *cov -= *e;
        Some((*cov, bound))
      })
      .filter(|(cov, _)| *cov == 0)
      .map(|(_, bound)| ((off + bound) / 2, (bound - off) / 2))
      .filter(|(x, y)| *x <= SIZE && *y <= SIZE)
      .next();
    if let Some((x, y)) = p {
      println!("{}", x * SIZE + y);
      break;
    }
  }
}