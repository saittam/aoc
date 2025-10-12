use std::io::BufRead;

fn f((x, y): (i64, i64)) -> f64 {
  x as f64 / y as f64
}

fn fix_sign((n, d): (i128, i128)) -> (i128, i128) {
  (n * d.signum(), d * d.signum())
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let stones = lines
    .map(|l| {
      let nv = l.split(|c: char| !c.is_numeric() && c != '-')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<i128>().expect("num"))
        .collect::<Vec<_>>();
      assert_eq!(nv.len(), 6);
      ((nv[0], nv[1], nv[2]), (nv[3], nv[4], nv[5]))
    })
    .collect::<Vec<_>>();

  // x1 + t1 vx1 = x2 + t2 vx2
  // t2 vx2 = x1 - x2 + t1 vx1
  // t2 = (x1 - x2 + t1 vx1) / vx2
  
  // y1 + t1 vy1 = y2 + t2 vy2
  // t1 vy1 = y2 - y1 + t2 vy2
  
  // t1 vy1 = y2 - y1 + (x1 - x2) vy2 / vx2 + t1 vx1 vy2 / vx2
  // t1 vy1 vx2 = (y2 - y1) vx2 + (x1 - x2) vy2 + t1 vx1 vy2
  // t1 (vy1 vx2 - vx1 vy2) = (y2 - y1) vx2 + (x1 - x2) vy2

  const MIN: i128 = 200000000000000;
  const MAX: i128 = 400000000000000;

  let n = stones.iter().zip(
    std::iter::successors(
      Some(stones.iter().skip(1)),
      |i| { let mut i = i.clone(); i.next(); Some(i) }))
    .flat_map(|(h1, i)| i.map(move |h2| (h1, h2)))
    .filter(|(((x1, y1, _), (vx1, vy1, _)),
              ((x2, y2, _), (vx2, vy2, _)))| {
      let t1 = ((y2 - y1) * vx2 + (x1 - x2) * vy2,
                vy1 * vx2 - vx1 * vy2);

      let t2 = ((y1 - y2) * vx1 + (x2 - x1) * vy1,
                vy2 * vx1 - vx2 * vy1);

      //println!("t1: {} t2: {}", f(t1), f(t2));

      if t1.1 == 0 || t2.1 == 0 {
        return false;
      }

      if t1.0.signum() * t1.1.signum() == -1 ||
        t2.0.signum() * t2.1.signum() == -1 {
        return false;
      }

      let i1x = (x1 * t1.1 + vx1 * t1.0, t1.1);
      let i1y = (y1 * t1.1 + vy1 * t1.0, t1.1);
      
      let i2x = (x2 * t2.1 + vx2 * t2.0, t2.1);
      let i2y = (y2 * t2.1 + vy2 * t2.0, t2.1);

      assert_eq!(i1x.0 * i2x.1, i2x.0 * i1x.1);
      assert_eq!(i1y.0 * i2y.1, i2y.0 * i1y.1);

      //println!("i1 {:?} {} {:?} {}", i1x, f(i1x), i1y, f(i1y));
      //println!("i2 {} {}", f(i2x), f(i2y));

      let ix = fix_sign(i1x);
      let iy = fix_sign(i1y);

      ix.0 >= MIN * ix.1 && ix.0 <= MAX * ix.1 &&
      iy.0 >= MIN * iy.1 && iy.0 <= MAX * iy.1
    })
    .count();

  println!("{}", n);
}