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

  const R: isize = 2000000;
  let mut c = s.iter()
    .map(|((x, y), r)| {
      let dx = r - (R - y).abs();
      (x - dx, x + dx)
    })
    .filter(|(l, u)| l < u)
    .flat_map(|(l, u)| [(l, 1), (u, -1)])
    .collect::<Vec<_>>();
  c.sort();
  let (r, _, _) = c.iter()
   .fold((0, isize::MIN, 0), |(c, p, l), (x, e)| 
          (c + (x - p) * (l > 0) as isize, *x, l + e));

  println!("{}", r);
}