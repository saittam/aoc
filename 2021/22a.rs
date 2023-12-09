use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let cubes = lines.take_while(|l| l.len() > 0)
    .map(|l| {
      let mut i = l.split_whitespace();
      let on = i.next() == Some("on");
      let n = i.next().unwrap().split(',')
        .map(|c| {
          let mut ni = c[2..]
            .split("..")
            .map(|n| n.parse::<i32>().unwrap());
          (ni.next().unwrap(), ni.next().unwrap())
        })
        .collect::<Vec<_>>();
      (on, (n[0], n[1], n[2]))
    })
    .collect::<Vec<_>>();

  let c = |n: &i32| i32::min(i32::max(*n, -50), 50);
  let mut s = [false; 101 * 101 * 101];
  for (on, ((xl, xh), (yl, yh), (zl, zh))) in &cubes {
    for (x, y, z) in (c(xl)..=c(xh)).flat_map(move |x|
                     (c(yl)..=c(yh)).flat_map(move |y|
                     (c(zl)..=c(zh)).map(move |z| (x, y, z)))) {
      s[(x + 50 + 101 * (y + 50 + 101 * (z + 50))) as usize] = *on;
    }
  }
  
  println!("{}", s.iter().filter(|o| **o).count());
}