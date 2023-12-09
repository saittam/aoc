use std::io::BufRead;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let map = lines
    .map(|l| {
      let mut p = l
        .split(',')
        .map(|w| w.parse::<isize>().expect("num"));
      (p.next().expect("x"),
       p.next().expect("y"),
       p.next().expect("z"))
    })
    .collect::<HashSet<_>>();

  let px = map.iter().map(|(x, _, _)| x);
  let xl = px.clone().min().expect("xl") - 1;
  let xh = px.clone().max().expect("xh") + 1;
  let py = map.iter().map(|(_, y, _)| y);
  let yl = py.clone().min().expect("yl") - 1;
  let yh = py.clone().max().expect("yh") + 1;
  let pz = map.iter().map(|(_, _, z)| z);
  let zl = pz.clone().min().expect("zl") - 1;
  let zh = pz.clone().max().expect("zh") + 1;

  let mut air = HashSet::new();
  let mut q = VecDeque::new();
  q.push_back((xl, yl, zl));
  let mut n = 0;
  while let Some(p) = q.pop_front() {
    let (x, y, z) = p;
    if map.contains(&p) {
      n += 1;
    } else if (xl..=xh).contains(&x) &&
      (yl..=yh).contains(&y) &&
      (zl..=zh).contains(&z) &&
      air.insert(p) {
      q.extend([
        (x - 1, y, z), (x + 1, y, z),
        (x, y - 1, z), (x, y + 1, z),
        (x, y, z - 1), (x, y, z + 1),
      ]);
    }
  }

  println!("{n}");
}