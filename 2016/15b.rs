use std::io::BufRead;

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
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut discs = lines
    .enumerate()
    .map(|(n, l)| {
      let mut wi = l
        .split(|c: char| !c.is_alphanumeric())
        .filter_map(|w| w.parse::<usize>().ok());
      assert_eq!(wi.next().expect("#"), n + 1);
      let cycle = wi.next().expect("cycle");
      assert_eq!(wi.next().expect("time"), 0);
      let phase = wi.next().expect("phase");
      (cycle, (phase + n + 1) % cycle)
    })
    .collect::<Vec<_>>();

  discs.push((11, (discs.len() + 1) % 11));

  let (_, p) = discs.iter().fold((1, 0), |(mc, mp), (dc, dp)| {
    let (gcd, mcinv) = eea(mc, *dc);
    let c = mc * *dc / gcd;
    let k = (((dc - dp) + dc * mc - mp) % dc) * mcinv;
    let p = (mp + k * mc) % c;
    (c, p)
  });

  println!("{}", p)
}