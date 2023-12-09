use std::io::BufRead;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::ops::RangeInclusive;

const SIZE: usize = 2;

fn dist(k: usize, a: usize)
  -> (usize, RangeInclusive<usize>) {
  let cpe = a * 2 + 2;
  let cp = [ 0, 1, 3, 5, 7, 9, 10 ];
  if cpe > cp[k] { (cpe - cp[k], (k + 1)..=(a + 1)) }
            else { (cp[k] - cpe, (a + 2)..=(k - 1)) }
}

fn diff(a: usize, b: usize) -> usize {
  (a as isize - b as isize).abs() as usize
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut rooms = [[None; SIZE]; 4];
  let ri = lines.skip(2).take(SIZE)
    .map(|l| l.chars().filter_map(|c| "ABCD".find(c))
                      .collect::<Vec<_>>());
  for (k, r) in ri.enumerate() {
    for i in 0..4 {
      rooms[i][k] = Some(r[i]);
    }
  }
  
  for r in &mut rooms {
    r.reverse();
  }
  
  let etable = [1, 10, 100, 1000];
  
  let mut mine = 0;
  for (a, r) in rooms.iter().enumerate() {
    for (rk, ra) in r.iter().enumerate()
                     .skip_while(|(_, ai)| **ai == Some(a)) {
      let ra = ra.unwrap();
      let d = (SIZE - rk) * etable[a] +
              (SIZE - rk) * etable[ra] +
              2 * diff(a, ra) * etable[ra];
      mine += d;
    }
  }
  
  let mut seen = HashSet::new();
  let mut q = BinaryHeap::new();
  seen.insert(([None; 7], rooms.clone()));
  q.push((Reverse(mine), [None; 7], rooms.clone()));
  while let Some((Reverse(e), c, rooms)) = q.pop() {
    let mut done = true;
    for (k, a) in c.iter().enumerate() {
      if let Some(a) = a {
        done = false;
        let r: &[Option<usize>; SIZE] = &rooms[*a];
        let kr = r.iter()
                  .position(|ai| *ai != Some(*a))
                  .unwrap();
        if r[kr] == None {
          let (_, mr) = dist(k, *a);
          if c[mr].iter().all(|ai| *ai == None) {
            let mut rc = c.clone();
            rc[k] = None;
            let mut rr = rooms.clone();
            rr[*a][kr] = Some(*a);
            if seen.insert((rc.clone(), rr.clone())) {
              q.push((Reverse(e), rc, rr));
            }
          }
        }
      }
    }
    
    for a in 0..4 {
      let r = &rooms[a];
      if let Some(kr) = (0..SIZE).rev()
                        .find(|k| r[*k].is_some()) {
        if r[0..=kr].iter().all(|ai| *ai == Some(a)) {
          continue;
        }
        done = false;
        let aa = r[kr].unwrap();
        for (ck, _) in c.iter().enumerate()
                        .filter(|(_, ca)| **ca == None) {
          let (d1, mr) = dist(ck, a);
          if c[mr].iter().all(|ai| *ai == None) {
            let mut rc = c.clone();
            rc[ck] = Some(aa);
            let mut rr = rooms.clone();
            rr[a][kr] = None;
            if seen.insert((rc.clone(), rr.clone())) {
              let (d2, _) = dist(ck, aa);
              let dd = 2 * diff(a, aa);
              let re = e + (d1 + d2 - dd) * etable[aa];
              q.push((Reverse(re), rc, rr));
            }
          }
        }
      }
    }
    
    if done {
      println!("{}", e);
      break;
    }
  }
} e + (d + SIZE - 1 - kr) * etable[aa];
            q.push((Reverse(re), rc, rr));
          }
        }
      }
    }
    
    if done {
      println!("{}", e);
      break;
    }
  }
}