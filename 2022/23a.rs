use std::io::BufRead;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut id = 0;
  let mut map = lines.by_ref()
    .take_while(|l| l.len() > 0)
    .enumerate()
    .flat_map(|(y, l)|
      l.chars()
        .enumerate()
        .filter_map(|(x, c)| if c == '#' {
            id += 1;
            Some(((x as isize, y as isize), id))
          } else {
            None
          })
        .collect::<Vec<_>>()
    )
    .collect::<HashMap<_, _>>();

  for k in 0..10 {
    let mut proposals = HashMap::new();
    let mut stay = HashSet::new();
'elves:
    for ((x, y), i) in &map {
      let ((x, y), i) = ((*x, *y), *i);
      let n = [
        (x - 1, y - 1),
        (x,     y - 1),
        (x + 1, y - 1),
        (x + 1, y    ),
        (x + 1, y + 1),
        (x,     y + 1),
        (x - 1, y + 1),
        (x - 1, y    ),
      ];
      if n.iter().any(|n| map.contains_key(n)) {
        for l in 0..4 {
          let nswe = [ 0, 4, 6, 2 ][(k + l) % 4];
          if n.iter().cycle().skip(nswe).take(3)
                     .all(|n| !map.contains_key(n)) {
            match proposals.entry(n[nswe + 1]) {
              Entry::Occupied(e) => {
                stay.extend(&[i, e.remove()]);
              }
              Entry::Vacant(e) => {
                e.insert(i);
              }
            }
            continue 'elves;
          }
        }
      }
      stay.insert(i);
    }
    proposals.extend(
      map.iter().filter(|(_, i)| stay.contains(i)));
    map = proposals;
  }

  let (xl, xh, yl, yh) = map.keys().copied().fold(
    (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
    |(xl, xh, yl, yh), (x, y)| (
      std::cmp::min(xl, x),
      std::cmp::max(xh, x),
      std::cmp::min(yl, y),
      std::cmp::max(yh, y)
    ));

  let r = (xh - xl + 1) * (yh - yl + 1) - map.len() as isize;
  println!("{r}");
}