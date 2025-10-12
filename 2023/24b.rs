use std::io::BufRead;

const EPS: f64 = 1e-12;

fn f64_eq(a: f64, b: f64, eps: f64) -> bool {
  let diff = (a - b).abs();
  a.is_finite() && b.is_finite() &&
  (diff <= eps || diff / (a.abs() + b.abs()) <= eps)
}

macro_rules! assert_feq_eps {
  ($a:expr, $b:expr, $eps:expr) => {
    let va = $a;
    let vb = $b;
    let veps = $eps;
    if !f64_eq(va, vb, veps) {
      panic!("{}={} != {}={} diff {} eps {}",
             stringify!($a), va, vb, stringify!($b),
             (va - vb).abs(), veps);
    }
  }
}

macro_rules! assert_feq {
  ($a:expr, $b:expr) => {
    assert_feq_eps!($a, $b, EPS);
  };
  ($a:expr, $b:expr, $eps:expr) => {
    assert_feq_eps!($a, $b, $eps);
  };
}

type V3 = (f64, f64, f64);

const ZERO: V3 = (0.0, 0.0, 0.0);

fn mulacc((x1, y1, z1): V3, s: f64, (x2, y2, z2): V3) -> V3 {
  (x1 + s * x2, y1 + s * y2, z1 + s * z2)
}

fn prod((x1, y1, z1): V3, (x2, y2, z2): V3) -> V3 {
  (x1 * x2, y1 * y2, z1 * z2)
}

fn cross_prod((x1, y1, z1): V3, (x2, y2, z2): V3) -> V3 {
  (y1 * z2 - y2 * z1, z1 * x2 - z2 * x1, x1 * y2 - x2 * y1)
}

fn dot_prod(a: V3, b: V3) -> f64 {
  csum(prod(a, b))
}

fn csum((x, y, z): V3) -> f64 {
  x + y + z
}

fn norm(v: V3) -> f64 {
  dot_prod(v, v).sqrt()
}

fn proj(v: V3, (e1, e2, e3): (V3, V3, V3)) -> V3 {
  (dot_prod(v, e1), dot_prod(v, e2), dot_prod(v, e3))
}

fn intersect_xy((p1, v1): (V3, V3), (p2, v2): (V3, V3))
  -> Option<((f64, V3), (f64, V3))> {
  let (x1, y1, _) = p1;
  let (x2, y2, _) = p2;
  let (vx1, vy1, _) = v1;
  let (vx2, vy2, _) = v2;
  
  let t1 = ((y2 - y1) * vx2 + (x1 - x2) * vy2) /
    (vy1 * vx2 - vx1 * vy2);

  let t2 = ((y1 - y2) * vx1 + (x2 - x1) * vy1) /
    (vy2 * vx1 - vx2 * vy1);

  if t1.is_nan() || t2.is_nan() {
    return None;
  }

  if t1 < 0.0 || t2 < 0.0 {
    return None;
  }

  let i1 = mulacc(p1, t1, v1);
  let i2 = mulacc(p2, t2, v2);
  
  if !i1.0.is_finite() ||
    !i1.1.is_finite() ||
    !i1.2.is_finite() ||
    !i2.0.is_finite() ||
    !i2.1.is_finite() ||
    !i2.2.is_finite() {
    println!("infinite i1 {:?} i2 {:?}", i1, i2);
    return None;
  }

  Some(((t1, i1), (t2, i2)))
}

fn lsfit(stones: &[(V3, V3)]) -> (V3, V3) {
  // Idea: Compute pairwise intersections of stones in x/y.
  // For each stone, keep the intersection with the smallest
  // z distance. The resulting points are somewhat close to
  // the actual rock collision points. Then, perform a least
  // squares fit to find approximate linear parameters.
  let dz = |((_, i1), (_, i2)): ((f64, V3), (f64, V3))|
    (i2.2 - i1.2).abs();
  let points = stones.iter()
    .flat_map(|s1| stones.iter()
              .filter_map(|s2| intersect_xy(*s1, *s2))
              .min_by(|a, b| dz(*a).partial_cmp(&dz(*b))
                                   .expect("cmp")))
      .map(|(p1, _)| p1)
      .collect::<Vec<_>>();

  // b + m * c + r = p
  // r = p - b - m * c
  // r² = (p - b - m * c)²
  // r² = (p - b)² - 2 (p - b) m * c + m² * c²
  // r² = p² - 2 p b + b² - 2 p m c + 2 b m c + m² c²
  // d r² / d b = -2 p + 2 b + 2 m c
  // d r² / d m = -2 p c + 2 b c + 2 m c²
  //
  // 0 = -si p + si b + si m c
  // n * b = si p - m si c
  //
  // 0 = -si p c + si b c + si m c²
  // b si c = si p c - m si c²
  //
  // (si p - m si c) si c = n si p c - n m si c²
  // si p si c - m (si c)² = n si p c - n m si c²
  // m (- (si c)² + n si c²) = n si p c - si p si c
  // m = (n si p c - si p si c) / (- (si c)² +  n si c²)

  let n = points.len() as f64;
  let (sp, sc, spc, scc) =
    points.into_iter().fold(
      (ZERO, 0.0, ZERO, 0.0), |(sp, sc, spc, scc), (t, i)| {
        let sp = mulacc(sp, 1.0, i);
        let sc = sc + t;
        let spc = mulacc(spc, t, i);
        let scc = scc + t * t;
        (sp, sc, spc, scc)
      });

  let mt = mulacc(mulacc(ZERO, n, spc), -sc, sp);
  let m = mulacc(ZERO, 1.0 / (-(sc * sc) + n * scc), mt);
  let bt = mulacc(sp, -sc, m);
  let b = mulacc(ZERO, 1.0 / n, bt);

  (b, m)
}

// Projects stone parameters into a basis with z in
// durection of m.
fn proj_stones(stones: &[(V3, V3)], m: V3)
  -> Vec<(V3, V3)> {
  // Compute basis vectors.
  let v1 = cross_prod(m, (m.1, -m.0, m.2));
  let v2 = cross_prod(m, v1);
  let basis = (mulacc(ZERO, 1.0 / norm(v1), v1),
               mulacc(ZERO, 1.0 / norm(v2), v2),
               ZERO);

  let meps = EPS * norm(m);
  assert_feq!(dot_prod(m, basis.0), 0.0, meps);
  assert_feq!(dot_prod(m, basis.1), 0.0, meps);
  assert_feq!(dot_prod(basis.0, basis.1), 0.0, meps);
  assert_feq!(norm(basis.0), 1.0);
  assert_feq!(norm(basis.1), 1.0);

  // Project the stone parameters.
  stones.iter()
    .map(|(s, vs)| (proj(*s, basis), proj(*vs, basis)))
    .collect::<Vec<_>>()
}

// Comoutes an error mesasure that indicates how far from
// the actual rock vector m is. This works by computing
// pairwise rock intersections in the plane perpendicular
// to m, then find the average distance of the intersection
// points from their center, i.e. average of the points.
fn proj_err(stones: &[(V3, V3)], m: V3) -> (f64, Vec<V3>) {
  let proj_stones = proj_stones(stones, m);

  // Compute pairwise intersections.
  let mut s2i = proj_stones.iter();
  let intersections = proj_stones.iter().zip(
    std::iter::from_fn(|| {
      s2i.next(); 
      Some(s2i.clone())
    }))
    .flat_map(|(s1, s2i)| s2i.map(|s2| (*s1, *s2)))
    .filter_map(|(s1, s2)| intersect_xy(s1, s2))
    .map(|((_, pi), _)| pi)
    .collect::<Vec<_>>();

  // Determine error.
  let scale = 1.0 / intersections.len() as f64;
  let center = intersections.iter()
    .fold(ZERO, |pc, pi| mulacc(pc, scale, *pi));
  let error = intersections.iter()
    .map(|pi| norm(mulacc(center, -1.0, *pi)))
    .sum::<f64>();

  (error * scale, intersections)
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let stones = lines
    .map(|l| {
      let nv = l.split(|c: char| !c.is_numeric() && c != '-')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<f64>().expect("num"))
        .collect::<Vec<_>>();
      assert_eq!(nv.len(), 6);
      ((nv[0], nv[1], nv[2]), (nv[3], nv[4], nv[5]))
    })
    .collect::<Vec<_>>();

  // Note that we don't really need the full set of input
  // stones for the algorithm to work - in my experience
  // truncating to the first 5 stones works well and speeds
  // up the computation significantly.
  let stones = &stones[0..usize::min(5, stones.len())];

  // First, perform a least squares fit to approximate
  // collision points to get a rough guess for the rock
  // parameters.
  let (_b, mut m) = lsfit(&stones);

  // Then, find the rock direction vector. The idea is that 
  // when projected into the plane perpendicular to the rock 
  // direction vector, all stone trajectories intersect in a
  // single point. If the vector deviates, the intersection
  // points will scatter, which can be used to obtain an
  // error measure. Then, we can perform a gradient descent
  // iteration to refine the approximate direction from above.
  let mut step = 1.0;
  for _ in 0..100 {
    let e = proj_err(&stones, m).0;
    //println!("error {:.6?} m {:.6?}", e, m);
    
    let delta = 0.5 * step;
    let de = [
      (1.0, 0.0, 0.0),
      (0.0, 1.0, 0.0),
      (0.0, 0.0, 1.0),
    ].into_iter()
      .map(|dir| {
        let ep = proj_err(&stones, mulacc(m, delta, dir)).0;
        let en = proj_err(&stones, mulacc(m, -delta, dir)).0;
        (ep - en) / (2.0 * delta)
      })
      .collect::<Vec<_>>();

    let de = (de[0], de[1], de[2]);
    let de1 = mulacc(ZERO, 1.0 / norm(de), de);
    step = f64::min(1.0 * e / norm(de), 0.5 * norm(m));
    //println!("e {:.6?} step {}", de, step);
    m = mulacc(m, -step, de1);
  }

  // m is rock direction vector, assuming the loop converged.
  // The intersections of two stones in the plane
  // perpendicular to m reveal collision times for these
  // stones. It is trivial to compute rock velocity and
  // initial position from them.
  let ps = proj_stones(&stones, m);
  let ((t1, _), (t2, _)) = intersect_xy(ps[0], ps[1])
    .expect("intersection");
  
  let t1 = t1.round();
  let t2 = t2.round();

  let (s1, sv1) = stones[0];
  let (s2, sv2) = stones[1];
  let i1 = mulacc(s1, t1, sv1);
  let i2 = mulacc(s2, t2, sv2);
  let di = mulacc(i2, -1.0, i1);
  let rv = mulacc(ZERO, 1.0 / (t2 - t1), di);
  let r = mulacc(i2, -t2, rv);

  //println!("r {:.5?} rv {:.5?}", r, rv);

  let n = r.0.round() + r.1.round() + r.2.round();
  println!("{}", n as i64);
}