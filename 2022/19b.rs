use std::io::BufRead;
use std::ops::{Add, Mul, Sub};

fn pw(l: &[usize; 4],
      f: &dyn Fn(usize, usize) -> usize,
      r: &[usize; 4]) -> [usize; 4] {
  [
    f(l[0], r[0]),
    f(l[1], r[1]),
    f(l[2], r[2]),
    f(l[3], r[3]),
  ]
}

const UVEC: [[usize; 4]; 4] = [
  [ 1, 0, 0, 0 ],
  [ 0, 1, 0, 0 ],
  [ 0, 0, 1, 0 ],
  [ 0, 0, 0, 1 ],
];

fn mp(cost: &[[usize; 4]; 4],
      lim: &[usize; 4],
      res: &[usize; 4],
      rob: &[usize; 4],
      best: usize,
      time: usize) -> usize {
  let ret = res[3] + time * rob[3];
  if ret + (time * (time - 1)) / 2 <= best {
    return ret;
  }
  
  cost.iter().enumerate().rev()
    .filter(|(i, _)| rob[*i] < lim[*i])
    .filter(|(_, c)| (0..4).all(|k| c[k] == 0 || rob[k] > 0))
    .fold(ret, |best, (i, c)| {
      let dt = c.iter().zip(res).zip(rob)
        .map(|((c, res), rob)|
          if c > res {
            (c - res + rob - 1) / rob
          } else {
            0
          })
        .max()
        .expect("dt") + 1;
      if dt >= time {
        return best;
      }
      let gain = pw(rob, &usize::mul, &[dt; 4]);
      let nres = pw(res, &usize::add, &gain);
      std::cmp::max(
        mp(cost, lim, &pw(&nres, &usize::sub, c),
           &pw(rob, &usize::add, &UVEC[i]), best, time - dt),
        best)
    })
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let cost = lines
    .enumerate()
    .map(|(k, l)| {
      let n = l
        .split(|c: char| !c.is_digit(10))
        .filter_map(|w| w.parse::<usize>().ok())
        .collect::<Vec<_>>();
      assert_eq!(n[0], k + 1);
      [
        [ n[1], 0, 0, 0 ],
        [ n[2], 0, 0, 0 ],
        [ n[3], n[4], 0, 0 ],
        [ n[5], 0, n[6], 0 ],
      ]
    })
    .collect::<Vec<_>>();

  const T: usize = 32;
  let n = cost[0..3].iter()
    .map(|c| {
      let l = c.iter()
        .fold([0, 0, 0, usize::MAX],
              |a, b| pw(&a, &std::cmp::max, b));
      mp(c, &l, &[0; 4], &[1, 0, 0, 0], 0, T)
    })
    .product::<usize>();
  
  println!("{n}");
}