use std::io::BufRead;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::borrow::Borrow;

macro_rules! fwd_ref_impl {
  ($op:ident, $f:ident, $lhs:ty, $rhs:ty, $out:ty) => {
    impl std::ops::$op<$rhs> for $lhs {
      type Output = $out;
      fn $f(self, rhs: $rhs) -> $out {
        self.borrow().$f(rhs.borrow())
      }
    }
  }
}

macro_rules! fwd_ref {
  ($op:ident<$rhs:ty> for $lhs:ty, $f:ident, $out:ty) => {
    fwd_ref_impl!($op, $f, $lhs, $rhs, $out);
    fwd_ref_impl!($op, $f, &$lhs, $rhs, $out);
    fwd_ref_impl!($op, $f, $lhs, &$rhs, $out);
  }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct V3([i32; 3]);

impl V3 {
  const ZERO: V3 = V3::new(0, 0, 0);

  const fn new(x: i32, y: i32, z: i32) -> V3 {
    V3([ x, y, z ])
  }
  
  fn comp_wise_1<F>(&self, mut f: F) -> V3
  where F: FnMut(i32) -> i32 {
    V3::new(f(self[0]), f(self[1]), f(self[2]))
  }

  fn comp_wise_2<F>(&self, r: &V3, mut f: F) -> V3
  where F: FnMut(i32, i32) -> i32 {
    V3::new(f(self[0], r[0]),
            f(self[1], r[1]),
            f(self[2], r[2]))
  }
  
  fn abs(&self) -> V3 {
    self.comp_wise_1(i32::abs)
  }
  
  fn cross_product(&self, r: &V3) -> V3 {
    V3::new(self[1] * r[2] - self[2] * r[1],
            self[2] * r[0] - self[0] * r[2],
            self[0] * r[1] - self[1] * r[0])
  }
  
  fn scalar_product(&self, r: &V3) -> i32 {
    self[0] * r[0] + self[1] * r[1] + self[2] * r[2]
  }
}

impl std::ops::Add<&V3> for &V3 {
  type Output = V3;
  fn add(self, r: &V3) -> V3 {
    self.comp_wise_2(r, i32::add)
  }
}
fwd_ref!(Add<V3> for V3, add, V3);

impl std::ops::Sub<&V3> for &V3 {
  type Output = V3;
  fn sub(self, r: &V3) -> V3 {
    self.comp_wise_2(r, i32::sub)
  }
}
fwd_ref!(Sub<V3> for V3, sub, V3);

impl std::ops::Index<usize> for V3 {
  type Output = i32;
  fn index(&self, i: usize) -> &i32 {
    &self.0[i]
  }
}

impl std::fmt::Debug for V3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
    -> std::fmt::Result {
    write!(f, "({},{},{})",
           self.0[0], self.0[1], self.0[2])
  }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct M3([i32; 9]);

impl M3 {
  const IDENTITY: M3 = M3([1, 0, 0,
                           0, 1, 0,
                           0, 0, 1]);

  fn from_rows(v: &[V3; 3]) -> M3 {
    M3([v[0][0], v[0][1], v[0][2],
        v[1][0], v[1][1], v[1][2],
        v[2][0], v[2][1], v[2][2]])
  }

  fn transpose(&self) -> M3 {
    let m = self.0;
    M3([m[0], m[3], m[6],
        m[1], m[4], m[7],
        m[2], m[5], m[8]])
  }
}

impl std::ops::Mul<&V3> for &M3 {
  type Output = V3;
  fn mul(self, r: &V3) -> V3 {
    let m = self.0;
    let v = r.0;
    let c = |n| (0..3).map(|k| m[n + k] * v[k]).sum();
    V3::new(c(0), c(3), c(6))
  }
}
fwd_ref!(Mul<V3> for M3, mul, V3);

impl std::ops::Mul<&M3> for &M3 {
  type Output = M3;
  fn mul(self, r: &M3) -> M3 {
    let l = self.0;
    let r = r.0;
    let p = |x: usize, y: usize| (0..3)
      .map(|k| l[3 * y + k] * r[x + 3 * k])
      .sum();
    M3([p(0, 0), p(1, 0), p(2, 0),
        p(0, 1), p(1, 1), p(2, 1),
        p(0, 2), p(1, 2), p(2, 2)])
  }
}
fwd_ref!(Mul<M3> for M3, mul, M3);

impl std::fmt::Debug for M3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
    -> std::fmt::Result {
    let m = self.0;
    write!(f, "({},{},{}; {},{},{}; {},{},{})",
           m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7], m[8])
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut sens = Vec::new();
  while lines.next().unwrap().len() > 0 {
    sens.push(lines
      .by_ref()
      .take_while(|l| l.len() > 0)
      .map(|s| {
        let mut i = s.split(',')
                     .map(|n| n.parse::<i32>().unwrap());
        V3::new(i.next().unwrap(),
                i.next().unwrap(),
                i.next().unwrap())
      })
      .collect::<Vec<_>>());
  }
  
  // Collect orientation invariant features for each
  // sensor. I'm using distance between pairs of beacons
  // as it is simple to compute and there aren't too many
  // beacons.
  let mut features = Vec::new();
  for s in &sens {
    let mut ds = BTreeMap::new();
    let mut ambi = HashSet::new();
    let mut i = s.iter().enumerate();
    while let Some((ka, a)) = i.next() {
      for (kb, b) in i.clone() {
        let mut d = (b - a).abs();
        d.0.sort();
        match ds.entry(d.clone()) {
          Entry::Vacant(e) => {
            e.insert((ka, kb));
          }
          Entry::Occupied(e) => {
            e.remove();
            ambi.insert(d);
          }
        }
      }
    }
    features.push(ds);
  }
  
  // Align pairs of sensors based on matching features.
  let mut alignments = Vec::new();
  let mut fiter = features.iter().enumerate();
  while let Some((kda, da)) = fiter.next() {
    for (kdb, db) in fiter.clone() {
      // Compute set of matching features between kda, kdb.
      let mut matching = Vec::new();
      let mut ia = da.iter();
      let mut ib = db.iter();
      let mut ea = ia.next();
      let mut eb = ib.next();
      while let (Some((va, kap)),
                 Some((vb, kbp))) = (ea, eb) {
        match va.cmp(vb) {
          Ordering::Less => ea = ia.next(),
          Ordering::Equal => {
            matching.push((kap, kbp));
            ea = ia.next();
            eb = ib.next();
          }
          Ordering::Greater => eb = ib.next(),
        }
      }
      
      // Determine transformation between each feature
      // pair. There can be false matches in case a
      // feature occurs for more than two sensors. Just
      // compute the transformation for all features and
      // take the one that's showing up most frequently.
      let mut trans = HashMap::new();
      for ((ka1, ka2), (kb1, kb2)) in &matching {
        let a1 = &sens[kda][*ka1];
        let a2 = &sens[kda][*ka2];
        let diffa = a2 - a1;
        
        // Note that it isn't clear which endpoint of the
        // feature is which. Try both polarities, take the
        // one that works.
        for (kb1f, kb2f) in &[(*kb1, *kb2), (*kb2, *kb1)] {
          let b1 = &sens[kdb][*kb1f];
          let b2 = &sens[kdb][*kb2f];
          let diffb = b2 - b1;
          let v0 = diffb.comp_wise_1(|c| match c {
            v if v == diffa[0] => 1,
            v if v == -diffa[0] => -1,
            _ => 0,
          });
          let v1 = diffb.comp_wise_1(|c| match c {
            v if v == diffa[1] => 1,
            v if v == -diffa[1] => -1,
            _ => 0,
          });
          let v2 = v0.cross_product(&v1);
          let rot = M3::from_rows(&[v0, v1, v2]);
          
          if diffa == &rot * diffb {
            let dist = a1 - &rot * b1;
            *trans.entry((dist, rot)).or_insert(0) += 1;
          }
        }
      }
      
      if let Some((t, n)) = trans.iter()
                                 .max_by_key(|(_, c)| *c) {
        alignments.push((Reverse(*n), kda, kdb, t.clone()));
      }
    }
  }
  alignments.sort();
  
  // Start with the scanner with the most overlaps.
  let (_, start) = (0..sens.len())
    .map(|k| (alignments
                .iter()
                .map(|(n, kda, kdb, _)|
                     if k == *kda || k == *kdb { n.0 }
                     else { 0 })
                .sum::<usize>(),
              k))
    .max().unwrap();
  
  // Iteratively add scanners and compute positions of
  // scanners and beacons relative to scanner |start|.
  let mut pos = vec![None; sens.len()];
  pos[start] = Some((V3::ZERO, M3:: IDENTITY));
  let mut beacons =
    sens[start].iter().cloned().collect::<HashSet<_>>();
  
  while pos.iter().any(Option::is_none) {
    let (_, ka, kb, (dist, rot)) = alignments
      .iter()
      .find(|(_, ka, kb, _)|
            pos[*ka].is_some() ^ pos[*kb].is_some())
      .unwrap();

    let (ka, kb, dist, rot) = if pos[*ka].is_some() {
      (*ka, *kb, dist.clone(), rot.clone())
    } else {
      let rot = rot.transpose();
      let dist = &rot * (V3::ZERO - dist);
      (*kb, *ka, dist, rot)
    };
    
    let (apos, arot) = pos[ka].clone().unwrap();
    let p = &apos + &arot * &dist;
    let rot = arot * rot;
    for b in &sens[kb] {
      beacons.insert(&p + &rot * b);
    }
    
    pos[kb] = Some((p, rot));        
  }
  
  let mut maxd = 0;
  let mut piter = pos.iter();
  while let Some(p1) = piter.next() {
    for p2 in piter.clone() {
      if let (Some((p1, _)), Some((p2, _))) = (p1, p2) {
        maxd = i32::max(maxd,
                        (p1 - p2).abs().0.iter().sum());
      }
    }
  }

  println!("{}", maxd);
}