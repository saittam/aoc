use std::io::BufRead;
use std::collections::VecDeque;
use std::collections::HashSet;

fn bit(pat: (&Vec<(u128, u128)>, &Vec<(u128, u128)>),
       len: (usize, usize),
       pos: (usize, usize),
       time: usize) -> bool {
  let shift = (time % len.0, time % len.1);
  let pat = (pat.0[pos.1], pat.1[pos.0]);
  pat.0.0 & (1 << ((shift.0 + pos.0) % len.0)) > 0 ||
  pat.0.1 & (1 << ((len.0 - shift.0 + pos.0) % len.0)) > 0 ||
  pat.1.0 & (1 << ((shift.1 + pos.1) % len.1)) > 0 ||
  pat.1.1 & (1 << ((len.1 - shift.1 + pos.1) % len.1)) > 0 ||
  false
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut hb = Vec::new();
  let w = lines.next().expect("top").len() - 2;
  let mut vb = vec![(0, 0); w];
  let lines = lines.take_while(
    |l| l.chars().filter(|c| *c == '#').count() == 2);
  let mut h = 0;
  for l in lines {
    let ci = l.chars().filter(|c| *c != '#').enumerate();
    let mut hbi = ['<', '>'].iter().map(
      |s| ci.clone().fold(
        0, |a, (x, c)| a | ((c == *s) as u128) << x));
    hb.push((hbi.next().unwrap(), hbi.next().unwrap()));
    for (x, c) in ci.clone() {
      vb[x].0 |= ((c == '^') as u128) << h;
      vb[x].1 |= ((c == 'v') as u128) << h;
    }
    h += 1;
  }

  let mut tcur = 0;
  let ends = [ (1, 1), (w, h) ];
  for i in 0..=2 {
    let (sx, sy) = ends[i % 2];
    let goal = ends[(i + 1) % 2];
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    loop {
      if q.len() == 0 {
        let tstart = ((tcur + 1)..)
          .find(|t| !bit((&hb, &vb), (w, h),
                         (sx - 1, sy - 1), *t))
          .expect("tstart");
        q.push_back(((sx, sy), tstart));
      };
      
      let ((x, y), t) = q.pop_front().expect("pop");
      if t > tcur {
        if !bit((&hb, &vb), (w, h), (sx - 1, sy - 1), t) {
          q.push_back(((sx, sy), t));
        }
        tcur = t;
        seen.clear();
      }
      if x < 1 || x > w || y < 1 || y > h {
        continue;
      }
      if bit((&hb, &vb), (w, h), (x - 1, y - 1), t) ||
         !seen.insert((x, y)) {
        continue;
      }

      if (x, y) == goal {
        tcur = t + 1;
        break;
      }
      q.extend(&[
        ((x, y - 1), t + 1),
        ((x - 1, y), t + 1),
        ((x, y), t + 1),
        ((x + 1, y), t + 1),
        ((x, y + 1), t + 1),
      ]);
    }
  }

  println!("{tcur}");
}