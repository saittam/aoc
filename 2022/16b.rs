use std::io::BufRead;
use std::collections::BinaryHeap;
use std::collections::HashMap;

struct IdMap(HashMap<String, u8>);

impl IdMap {
  fn new() -> IdMap {
    IdMap(HashMap::new())
  }

  fn get(&mut self, l: &str) -> u8 {
    let n = self.0.len() as u8;
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
      .and_then(|s| s.parse::<u16>().ok())
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

  const T: u16 = 26;
  let maxr = valves.values().map(|(r, _)| r).sum::<u16>();

  let mut me = 0;
  let mut q = BinaryHeap::new();
  let aa = ids.get("AA");
  q.push((maxr * T, (0, aa), (0, aa), 0, 0u128));
  while let Some((tot, (t1, l1), s2, rate, open)) = q.pop() {
    if t1 > me {
      println!("{} {:?} {:?} {} {} {}", tot, (t1, l1), s2, rate, maxr, q.len());
      me = t1;
    }
    if t1 == T {
      println!("{}", tot);
      break;
    }

    let (vrate, conn) = &valves[&l1];
    let open = open | (1 << l1);
    let rate = rate + vrate; 
    if rate == maxr {
      println!("{}", tot);
      break;
    }
    let mut wait = true;
    for (next, dist) in conn.iter()
      .filter(|(v, d)| ((1 << *v) & open == 0) &&
                       (**v != s2.1) &&
                       (t1 + **d + 1 <= T)) {
      if l1 == aa && s2.1 != aa && s2.1 > *next {
        continue;
      }
      assert!(*next != s2.1);
      let s1 = (t1 + *dist + 1, *next);
      let (s1, s2) =
        if s1 < s2 { (s1, s2) } else { (s2, s1) };
      //println!("{} {} {} {}", maxr, rate, s1.0, t1);
      let ntot = tot - (maxr - rate) * (s1.0 - t1);
      q.push((ntot, s1, s2, rate, open));
      wait = false;
    }
    if wait {
      let ntot = tot - (maxr - rate) * (s2.0 - t1);
      q.push((ntot, s2, (T, l1), rate, open));
    }
  }
}