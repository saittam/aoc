use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut machines = Vec::new();
  loop {
    let numbers = lines.by_ref()
      .take(3)
      .fold(Vec::new(),
            |mut n, l| { 
              n.extend(
                l.split(|c: char| !c.is_digit(10))
                .filter_map(|w| w.parse::<i32>().ok()));
              n
            });
    if let [ax, ay, bx, by, px, py] = numbers.as_slice() {
      machines.push((*ax, *ay, *bx, *by, *px, *py));
    } else {
      panic!("bad machine spec");
    }

    if lines.next().is_none() {
      break;
    }
  }
  
  // na * ax + nb * bx = px
  // nb = (px - na * ax) / bx
  // na * ay + (px - na * ax) * by / bx = py
  // na * ay + px * by / bx - na * ax * by / bx = py
  // na * (ay - (ax * by) / bx) = py - px * by / bx
  // na = (py * bx - px * by) / (ay * bx - ax * by)

  let n = machines.iter()
    .copied()
    .filter_map(|(ax, ay, bx, by, px, py)| {
      let d = ay * bx - ax * by;
      assert!(d != 0); // colinear vectors
      let na = (py * bx - px * by) / (ay * bx - ax * by);
      let nb = if bx != 0 {
        (px - na * ax) / bx
      } else {
        (py - na * ay) / by
      };
      (na * ax + nb * bx == px &&
       na * ay + nb * by == py)
      .then_some(3 * na + nb)
    })
    .sum::<i32>();
  
  println!("{n}");
}