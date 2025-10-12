use std::io::BufRead;
use std::collections::HashSet;

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
  let s = [(start)].into_iter().collect::<HashSet<_>>();
  let (_, _, n, _) = (0..64).fold(
    (s, HashSet::new(), 1, 0),
    |(s1, s2, n1, n2), _| {
      let s = s1.iter().cloned().flat_map(|(x, y)|
        [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)])
        .filter(|p| map.contains(p))
        .filter(|p| !s2.contains(p))
        .collect::<HashSet<_>>();
      let n = n2 + s.len();
      (s, s1, n, n1)
    });

  println!("{}", n);
}