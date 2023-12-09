use std::io::BufRead;
use std::collections::HashMap;

// 0 1 2   6 3 0   3(2 - x) + y  3(2 - y) + x
// 3 4 5   7 4 1
// 6 7 8   8 5 2
fn transform<F>(n: usize, p: u16, f: F) -> u16
where F: Fn(usize, usize) -> usize {
  (0..(n * n)).fold(0, |t, i| {
    let pbit = 1 << f(i % n, i/ n);
    let tbit = ((p & pbit) != 0) as u16;
    t | (tbit << i)
  })
}

fn rot(n: usize, p: u16) -> u16 {
  transform(n, p, |x, y| n * (n - 1 - x) + y)
}

fn flip(n: usize, p: u16) -> u16 {
  transform(n, p, |x, y| n * (n - 1 - y) + x)
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let (p2, p3) = lines
    .map(|l| {
      let mut pi = l.split(" => ")
        .map(|w| w.chars()
                  .filter_map(|c| match c {
                    '.' => Some(0),
                    '#' => Some(1),
                    _ => None,
                  })
                  .enumerate()
                  .fold((0, 0),
                        |(_, p), (i, b)| (i, p | (b << i))));
      (pi.next().expect("pattern"),
       pi.next().expect("refinement"))
    })
    .fold((HashMap::new(), HashMap::new()),
          |(mut p2, mut p3), ((kp, p), (kr, r))| {
            let (n, pm) = match kp {
              3 => (2, &mut p2),
              8 => (3, &mut p3),
              _ => panic!("bad size"),
            };
            assert_eq!(kr + 1, (n + 1) * (n + 1));
            let mut pr = p;
            pm.extend(
              std::iter::from_fn(
                || { pr = rot(n, pr); Some(pr) })
              .flat_map(|p| [p, flip(n, p)])
              .map(|p| (p, r))
              .take(8));
            (p2, p3)
          });

  let mut ts = 3;
  let mut size = 3;
  let mut img = vec![ 0b_111_100_010u16 ];
  for _ in 0..18 {
    let (pts, pn, pm) = if size % 2 == 0 {
      (2, size / 2, &p2)
    } else if size % 3 == 0 {
      (3, size / 3, &p3)
    } else {
      panic!("bad size {}", size)
    };
    let tn = size / ts;
    let nimg = (0..(pn * pn))
      .map(|i| (i % pn, i / pn))
      .map(|(px, py)| {
        let (pxl, pyl) = (px * pts, py * pts);
        let (pxh, pyh) = (pxl + pts, pyl + pts);
        let p = [
          (pxl, pyl),
          (pxh - 1, pyl),
          (pxl, pyh - 1),
          (pxh - 1, pyh - 1)
        ].into_iter()
          .map(|(x, y)| (x / ts, y / ts))
          .map(|(tx, ty)| {
            let (txl, tyl) = (tx * ts, ty * ts);
            let (txh, tyh) = (txl + ts, tyl + ts);
            let (rxl, ryl) = (txl.max(pxl), tyl.max(pyl));
            let (rxh, ryh) = (txh.min(pxh), tyh.min(pyh));

            let timg = img[ty * tn + tx];
            let prm = ((1 << (rxh - rxl)) - 1) << (rxl - pxl);
            (ryl..ryh)
              .map(|ry| {
                  let trow = timg >> ((ry - tyl) * ts);
                  let prow =
                    (trow >> (rxl - txl)) << (rxl - pxl);
                  (prow & prm) << ((ry - pyl) * pts)
                })
              .fold(0, |p, r| p | r)
          })
          .fold(0, |t, p| t | p);
        pm[&p]
      })
      .collect::<Vec<_>>();

    ts = pts + 1;
    size = ts * pn;
    img = nimg;
  }
  
  let n = img.iter().map(|p| p.count_ones()).sum::<u32>();
  println!("{}", n);
}