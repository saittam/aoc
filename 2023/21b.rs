use std::io::BufRead;
use std::collections::HashSet;

type Pos = (isize, isize);

// A helper struct to compute reachable position (counts).
// Ideally this would implement Iterator, but it would have
// to return references to itself, which is not possible.
struct PosIter<'a> {
  dim: isize,
  valid: &'a HashSet<Pos>,
  front1: HashSet<Pos>,
  count1: usize,
  front2: HashSet<Pos>,
  count2: usize,
}

impl<'a> PosIter<'a> {
  fn new(valid: &'a HashSet<Pos>, dim: isize, start: Pos)
    -> Self {
    PosIter {
      dim,
      valid,
      front1: HashSet::from([start]),
      count1: 1,
      front2: HashSet::new(),
      count2: 0,
    }
  }

  // Expand the frontier. By only tracking the frontier, we
  // avoid recomputing previously reached positions over and
  // over again.
  fn next(&mut self) {
    let front0 = self.front1.iter()
      .cloned()
      .flat_map(|(x, y)|
        [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)])
      .filter(|(x, y)| self.valid.contains(
        &(x.rem_euclid(self.dim), y.rem_euclid(self.dim))))
      .filter(|p| !self.front2.contains(p))
      .collect::<HashSet<_>>();
    let count0 = self.count2 + front0.len();
    std::mem::swap(&mut self.front2, &mut self.front1);
    self.count2 = self.count1;
    (self.front1, self.count1) = (front0, count0);
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut start = None;
  let map = lines
    .enumerate()
    .flat_map(|(y, l)| l.chars()
      .enumerate()
      .map(move |(x, c)| ((x as isize, y as isize), c))
      .collect::<Vec<_>>())
    .filter_map(|(p, c)| Some(match c {
      '.' => p,
      'S' => {
        start = Some(p);
        p
      }
      '#' => return None,
      _ => panic!("bad tile {}", c),
    }))
    .collect::<HashSet<_>>();

  let start = start.expect("start");
  let w = map.iter().map(|(x, _)| *x).max().expect("w") + 1;
  let h = map.iter().map(|(_, y)| *y).max().expect("h") + 1;

  // Check geometry assumptions. Unequal dimensions and
  // off-center starting point can in principle be handled
  // by using LCM and wrapping, but the input appears to be
  // constructed to provide a square and start in center.
  assert_eq!(w, h);
  let dim = w;
  let center = (w - 1) / 2;
  assert_eq!(start, (center, center));

  // After 2 * dim iterations, we are back at a similar
  // state regarding partially filled tiles at the corners
  // and edges, but with everything expanded. Thus, there
  // is some periodicity and the interesting points in time
  // are the ones in phase with the target time (mod period).
  const N: isize = 26501365;
  let period = 2 * dim;
  let phase = N % period;

  // Get the number of positions for a completely filled
  // tile. There are two states that alternate (corresponding
  // to even and odd travel distance). These values are 
  // determining how many positions get added in each
  // expansion cycle.
  let mut piter = PosIter::new(&map, dim, start);
  let (mut n1, mut n2) = (0, 0);
  loop {
    let tdim = 0..dim;
    let n = n2 + piter.front1.iter()
      .filter(|(x, y)| tdim.contains(x) && tdim.contains(y))
      .count();
    if n == n2 {
      break;
    }
    (n1, n2) = (n, n1);

    piter.next();
  }

  // Main loop: Iteratively compute the number of positions
  // with increasing time. Whenever phase aligns with the
  // target time, look at the position count and compare
  // it with what would be expected by adding the new tiles
  // due to expansion to the previous count. Once things
  // have stabilized and the predictions are accurate, we
  // have the coefficients for computing the count at the
  // target time.
  let mut piter = PosIter::new(&map, dim, start);
  let mut phasecount = 0;
  let mut correction = 0;
  for k in 0.. {
    if k % period == phase {
      // Every expansion adds 8n + 4 tiles. Most of them
      // are filled, tiles at the edge are only partially 
      // filled, the number of missing positions is constant.
      // The 8n + 4 added tiles are actually 4n plus (4n - 4),
      // for each of the two alternating fill patterns. Their
      // difference is constant though, so can be covered by 
      // the correction value that we need to account for the 
      // partially filled edge tiles anyways. With this,
      // each expansion adds 8n * (n1 + n2) / 2 positions,
      // minus a constant. We can extract this correction
      // by looking at actual position counts for successive
      // expansions.
      let tile_n = k / dim;
      let n12 = (n1 + n2) as isize;
      let guess = phasecount + 4 * tile_n * n12;
      let ncount = piter.count1 as isize;
      if ncount + correction == guess {
        // Constant from previous expansion is correct.
        // It won't change for the remaining expansions,
        // so use it to compute the target time count by
        // finding the number of tiles added on thw remaining
        // expansions and computing the number of positions
        // added due to that, minus the sum of correction
        // values for the remaining expansions. Essentially,
        // we are evaluating a quadratic equation, but it
        // instead of just blindlt fitting its coefficients, 
        // it's nice to see where things come from ;-)
        let cycles = (N - k) / period;
        let tiles_added =
          4 * (cycles * tile_n + cycles * (cycles + 1));
        let target_count =
          ncount + tiles_added * n12 - cycles * correction;
        println!("{}", target_count);
        break;
      }
      correction = guess - ncount;
      phasecount = ncount;
    }

    piter.next();
  }
}