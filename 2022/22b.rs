use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
enum Tile {
  Air,
  Rock,
}

#[derive(Clone, Copy, Debug)]
enum Step {
  Left,
  Right,
  Forward,
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let map = lines.by_ref()
    .take_while(|l| l.len() > 0)
    .enumerate()
    .flat_map(|(y, l)|
      l.chars()
        .enumerate()
        .filter_map(move |(x, c)| match c {
          '.' => Some(Tile::Air),
          '#' => Some(Tile::Rock),
          _ => None,
        }.map(|t| ((x as isize + 1, y as isize + 1), t)))
      .collect::<Vec<_>>()
    )
    .collect::<HashMap<_, _>>();

  let n = ((map.len() / 6) as f64).sqrt() as isize;
  let n1 = n - 1;
  let startx = map.keys()
    .filter(|(_, y)| *y == 1)
    .map(|(x, _)| *x)
    .min()
    .expect("startx");

  let mut edges = HashMap::new();
  let mut q = VecDeque::new();
  let mut seen = HashSet::new();
  q.push_back(((startx, 1), [0b011, 0b111, 0b101, 0b001]));
  while let Some(((x, y), c)) = q.pop_front() {
    if !map.contains_key(&(x, y)) || !seen.insert((x, y)) {
          continue;
    }
    let cor = c[0] | c[1] | c[2] | c[3];
    let cand = c[0] & c[1] & c[2] & c[3];
    let cflip = !(cor ^ cand) & 0b111;
    let cp =
      [(x, y), (x + n1, y), (x + n1, y + n1), (x, y + n1)];
    let np = [(x, y - n), (x + n, y), (x, y + n), (x - n, y)];
    let ci = c.iter().copied().zip(cp);
    let ni = ci.clone().zip(ci.cycle().skip(1)).zip(np);
    for (k, (((c1, cp1), (c2, cp2)), np)) in ni.enumerate() {
      let pd = ((cp2.0 - cp1.0).signum(),
                (cp2.1 - cp1.1).signum());
      edges.insert((c1, c2), (cp1, pd));
      let mut c = [ c1 ^ cflip, c2 ^ cflip, c2, c1 ];
      c.rotate_right(k);
      q.push_back((np, c));
    }
  }
  assert_eq!(edges.len(), 24);
  
  let seams = edges.iter()
    .map(|((c1, c2), (ip, id))| {
      let tp = ((ip.0 - 1) / n, (ip.1 - 1) / n);
      let (op, od) = edges[&(*c2, *c1)];
      let d = (id.1, id.0 * -1);
      ((tp, d), (ip, op, od))
    })
    .collect::<HashMap<_, _>>();

  let mut steps = Vec::new();
  let mut nfwd = 0usize;
  for c in lines.next().expect("path").chars() {
    let s = match c {
      'R' => Step::Right,
      'L' => Step::Left,
      d if c.is_digit(10) => {
        nfwd = nfwd * 10 +
          d.to_digit(10).expect("digit") as usize;
        continue;
      }
      _ => panic!("step {c}"),
    };
    steps.extend(std::iter::repeat(Step::Forward).take(nfwd));
    nfwd = 0;
    steps.push(s);
  }
  steps.extend(std::iter::repeat(Step::Forward).take(nfwd));

  let ((x, y), d) = steps.iter()
    .scan(((startx, 1), (1, 0)), |(p, d), s| {
      let (pn, dn) = match s {
        Step::Forward => {
          let pn = (p.0 + d.0, p.1 + d.1);
          let (pn, t, dn) = if let Some(t) = map.get(&pn) {
            (pn, t, *d)
          } else {
            let tp = ((p.0 - 1) / n, (p.1 - 1) / n);
            let (ip, op, od) = seams[&(tp, *d)];
            let k = n1 - std::cmp::max((p.0 - ip.0).abs(),
                                       (p.1 - ip.1).abs());
            let pn = (op.0 + k * od.0, op.1 + k * od.1);
            let dn = (od.1 * -1, od.0);
            (pn, map.get(&pn).expect("out"), dn)
          };
          match t {
            Tile::Air => (pn, dn),
            Tile::Rock => (*p, *d),
          }
        }
        Step::Left => (*p, (d.1, d.0 * -1)),
        Step::Right => (*p, (d.1 * -1, d.0)),
      };
      *p = pn;
      *d = dn;
      Some((pn, dn))
    })
    .last()
    .expect("last");
  let di = [(1, 0), (0, 1), (-1, 0), (0, -1)].iter()
    .position(|v| *v == d)
    .expect("dir");
   println!("{}", 1000 * y + 4 * x + di as isize);
}