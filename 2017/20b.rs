use std::io::BufRead;
use std::collections::HashMap;

type V3 = (i64, i64, i64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Sol {
  None,
  One(i64),
  Two(i64, i64),
  All,
}

impl Sol {
  fn add(&self, n: i64, d: i64) -> Sol {
    let (q, r) = (n / d, n % d);
    if r != 0 {
      *self
    } else {
      match self {
        Sol::None => Sol::One(q),
        Sol::One(v) => Sol::Two(*v, q),
        _ => panic!("add"),
      }
    }
  }

  fn has(&self, val: i64) -> bool {
    match self {
      Sol::None => false,
      Sol::One(v) => val == *v,
      Sol::Two(v1, v2) => val == *v1 || val == *v2,
      Sol::All => true,
    }
  }
  
  fn and(&self, other: &Sol) -> Sol {
    match self {
      Sol::None => Sol::None,
      Sol::One(v) =>
        if other.has(*v) { *self } else { Sol::None },
      Sol::Two(v1, v2) =>
        match (other.has(*v1), other.has(*v2)) {
          (true, true) => *self,
          (true, false) => Sol::One(*v1),
          (false, true) => Sol::One(*v2),
          _ => Sol::None,
        },
      Sol::All => *other,
    }
  }
}
  
fn qroot(a: i64, b: i64, c: i64) -> Sol {
  if a != 0 {
    let d = b * b - 4 * a * c;
    if d < 0 {
      return Sol::None;
    } else if d == 0 {
      Sol::None.add(-b, 2 * a)
    } else {
      let r = (d as f64).sqrt() as i64;
      Sol::None.add(-b + r, 2 * a).add(-b - r, 2 * a)
    }
  } else if b != 0 {
    Sol::None.add(-c, b)
  } else if c != 0 {
    Sol::None
  } else {
    Sol::All
  }
}

fn collide((p1, v1, a1): &(V3, V3, V3),
           (p2, v2, a2): &(V3, V3, V3)) -> Option<u64> {
  // p v a
  // p+v+a v+a a
  // p+2v+3a v+2a a
  // p+3v+6a v+3a a
  // ...
  // p + t * v + (t * (t + 1)) / 2 * a 
  //
  // p + tv + ½t²a + ½ta
  //
  // ½at² + (½a + v)t + p = ½At² + (½A + V)t + P
  // (A-a)t² + (A-a + 2(V-v))t + 2(P-p) = 0

  let comps = |dp, dv, da| qroot(da, da + 2 * dv, 2 * dp);

  let s0 = comps(p1.0 - p2.0, v1.0 - v2.0, a1.0 - a2.0);
  let s1 = comps(p1.1 - p2.1, v1.1 - v2.1, a1.1 - a2.1);
  let s2 = comps(p1.2 - p2.2, v1.2 - v2.2, a1.2 - a2.2);
  
  match s0.and(&s1).and(&s2) {
    Sol::All => Some(0),
    Sol::One(v) if v >= 0 => Some(v as u64),
    Sol::Two(v1, v2) if v1 >= 0 && v2 >= 0 =>
      Some(i64::min(v1, v2) as u64),
    Sol::Two(v1, _) if v1 >= 0 => Some(v1 as u64),
    Sol::Two(_, v2) if v2 >= 0 => Some(v2 as u64),
    _ => None,
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  assert_eq!(qroot(1, 0, 0), Sol::One(0));
  assert_eq!(qroot(1, 2, 1), Sol::One(-1));
  assert_eq!(qroot(1, -2, 1), Sol::One(1));
  assert_eq!(qroot(1, 3, 2), Sol::Two(-1, -2));
  assert_eq!(qroot(0, 1, 2), Sol::One(-2));
  assert_eq!(qroot(0, 0, 1), Sol::None);
  
  let particles = lines
    .map(|l| {
      let mut numi = l
        .split(|c: char| c != '-' && !c.is_numeric())
        .filter_map(|p| p.parse::<i64>().ok());

      let mut triple = || (numi.next().expect("1"),
                           numi.next().expect("2"),
                           numi.next().expect("3"));

      (triple(), triple(), triple())
    })
    .collect::<Vec<_>>();

  let mut ni = 0..particles.len();
  let nii = std::iter::from_fn(|| {
    ni.next();
    Some(ni.clone())
  });
  let mut collisions = (0..particles.len()).zip(nii)
    .flat_map(|(i, ji)| ji.map(move |j| (i, j)))
    .filter_map(|(i, j)| collide(&particles[i], &particles[j])
                           .map(|t| (t, i, j)))
    .collect::<Vec<_>>();
  collisions.sort();

  let mut destroyed = HashMap::new();
  for (t, i, j) in &collisions {
    if destroyed.get(i).unwrap_or(t) == t &&
      destroyed.get(j).unwrap_or(t) == t {
      destroyed.extend([(i, t), (j, t)]);
    }
  }

  println!("{:?}", particles.len() - destroyed.len());
}