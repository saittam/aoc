use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

fn neigh((x, y): (usize, usize)) -> [(usize, usize); 4] {
  [ (x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y) ]
}

fn flip(d: usize) -> usize {
  (d + 2) % 4
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut start = None;
  let map = lines
    .enumerate()
    .flat_map(|(y, l)| l.chars().enumerate().filter_map(
      |(x, c)| Some(((x + 1, y + 1), match c {
        '-' => 0b1010,
        '|' => 0b0101,
        'L' => 0b0011,
        'F' => 0b0110,
        '7' => 0b1100,
        'J' => 0b1001,
        '.' => return None,
        'S' => {
          start = Some((x + 1, y + 1));
          return None;
        }
        _ => panic!("Ceci n'est pas une pipe: {}", c)
      }))).collect::<Vec<_>>())
    .collect::<HashMap<_, _>>();

  let start = start.expect("start");
  
  let mut ni = neigh(start)
    .into_iter()
    .enumerate()
    .filter(|(d, p)| {
      let ic = 1 << flip(*d);
      matches!(map.get(&p), Some(&mc) if mc & ic != 0)
    });
  
  let (id, _) = ni.next().expect("id");
  let (od, _) = ni.next().expect("od");
  let mut pos = start;
  let mut od = od;
  
  let (o, qr, ql, lt) = [(pos, id, od)].into_iter().chain(
    std::iter::from_fn(|| {
      pos = neigh(pos)[od];
      let pc = map.get(&pos)?;
      let id = flip(od);
      od = ((1usize << id) ^ pc).trailing_zeros() as usize;
      Some((pos, id, od))
    }))
    .fold((0, Vec::new(), Vec::new(), HashSet::new()),
          |(o, mut qr, mut ql, mut lt), (p, id, od)| {
            lt.insert(p);
            let ot = ((od + 4 - id) % 4) as isize - 2;
            let n = neigh(p);
            let od = if id < od { od } else { od + 4 };
            ql.extend(((id + 1)..od).map(|d| n[d % 4]));
            qr.extend(((od + 1)..(id + 4)).map(|d| n[d % 4]));
            (o + ot, qr, ql, lt)
          });

  assert_eq!(o.abs(), 4);
  let mut q = if o > 0 { qr } else { ql };
  let mut inside = HashSet::new();
  while let Some(p) = q.pop() {
    if !lt.contains(&p) && inside.insert(p) {
      q.extend(neigh(p));
    }
  }
  
  println!("{}", inside.len());
}