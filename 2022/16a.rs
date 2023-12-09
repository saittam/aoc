use std::io::BufRead;
use std::collections::BinaryHeap;
use std::collections::HashMap;

struct IdMap(HashMap<String, usize>);

impl IdMap {
  fn new() -> IdMap {
    IdMap(HashMap::new())
  }

  fn get(&mut self, l: &str) -> usize {
    let n = self.0.len();
    *self.0.entry(l.to_string()).or_insert(n)
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut ids = IdMap::new();

  let mut valves = HashMap::new();
  for l in lines {
    let mut w = l
      .split(|c: char| !c.is_alphanumeric())
      .filter(|s| s.len() > 0);
    let v = ids.get(w.nth(1).expect("valve"));
    let r = w.nth(3)
      .and_then(|s| s.parse::<usize>().ok())
      .expect("rate");
    let c = w.skip(4)
      .map(|l| (ids.get(l), 1))
      .collect::<HashMap<_, _>>();
    valves.insert(v, (r, c));
  }

  let mut more = true;
  while more {
    let mut vn = valves.clone();
    for (s, (_, c)) in &valves {
      for (i, d1) in c {
        for (e, d2) in &valves[&i].1 {
          let d = vn.get_mut(s).unwrap().1
            .entry(*e).or_insert(d1 + d2);
          *d = std::cmp::min(*d, d1 + d2);
        }
      }
    }
    more = vn != valves;
    valves = vn;
  }

  let valves = valves.iter()
    .map(|(v, (r, c))| {
      let cwithr = c.iter()
        .filter(|(v, _)| valves[v].0 > 0)
        .map(|(v, d)| (*v, *d))
        .collect::<HashMap<_, _>>();
      (v, (*r, cwithr))
    })
    .collect::<HashMap<_, _>>();

  const T: usize = 30;
  let maxr = valves.values().map(|(r, _)| r).sum::<usize>();

  let mut q = BinaryHeap::new();
  q.push((maxr * T, 0, 0, ids.get("AA"), 0u128));
  while let Some((tot, time, rate, loc, open)) = q.pop() {
    if time == T {
      println!("{}", tot);
      break;
    }

    let (vrate, conn) = &valves[&loc];
    let open = open | (1 << loc);
    let rate = rate + vrate; 
    if rate == maxr {
      println!("{}", tot);
      break;
    }
    for (next, dist) in conn.iter()
      .filter(|(v, _)| (1 << *v) & open == 0) {
      let dur = *dist + 1;
      if time + dur <= T {
        let ntot = tot - (maxr - rate) * dur;
        q.push((ntot, time + dur, rate, *next, open));
      } else {
        let ntot = tot - (maxr - rate) * (T - time);
        q.push((ntot, T, rate, loc, open));
      }
    }
  }
}