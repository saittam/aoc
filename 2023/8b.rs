use std::io::BufRead;
use std::collections::HashMap;

fn eea(a: usize, b: usize) -> (usize, usize) {
  let mut r = (b, a);
  let mut s = (0, 1);
  while r.0 > 0 {
    let (div, rem) = (r.1 / r.0, r.1 % r.0);
    r = (rem, r.0);
    s = (s.1 - div as isize * s.0, s.0);
  }
  (r.1, s.1.rem_euclid(b as isize) as usize)
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let turns = lines.next().expect("turns").chars()
    .map(|c| match c {
      'L' => 0,
      'R' => 1,
      _ => panic!("bad turn"),
    })
    .collect::<Vec<_>>();

  let graph = lines.skip(1)
    .map(|l| {
      let mut ni = l.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_owned);
      (ni.next().expect("node"), ni.collect::<Vec<_>>())
    })
    .collect::<HashMap<_, _>>();

  let (n, _) = graph.keys()
    .filter(|n| n.ends_with("A"))
    .fold((0, 1), |(tp, tc), start| {
    let mut pi = turns.iter().cycle().scan(start, |pos, t| {
      *pos = graph.get(*pos)?.get(*t)?;
      Some(*pos)
    });
    let gp = pi.position(|p| p.ends_with("Z")).expect("Z") + 1;
    let gc = pi.position(|p| p.ends_with("Z")).expect("Z") + 1;
    // tp + k * tc == gp + l * gc
    // tp + k * tc == gp   (mod gc)
    // k = (gp - tp) * inv(tc)
    let (gcd, tcinv) = eea(tc, gc);
    let k = ((gp + (tc * gc) - tp % tc) % gc) * tcinv;
    let c = (tc * gc) / gcd;
    let mut p = (tp % tc) + k * tc;
    while p < gp || p < tp {
      p += c;
    }
    (p, c)
  });
 
  println!("{}", n);
}