use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines()
    .map(|r| r.unwrap())
    .take_while(|l| l.len() > 0)
    .collect::<Vec<_>>();

  let mut map = lines.iter()
    .enumerate()
    .flat_map(|(y, l)| l.chars().enumerate()
      .map(move |(x, c)| ((x as isize, y as isize),
                          c as usize)))
    .collect::<HashMap<_, _>>();

  let mut q = VecDeque::new();
  let mut seen = HashSet::new();
  let ps = *map.iter()
    .find(|(_, h)| **h == ('E' as usize))
    .expect("start").0;
  map.insert(ps, 'z' as usize);
  q.push_back((ps, 0));
  seen.insert(ps);
'outer:
  while let Some(((x, y), d)) = q.pop_front() {
    let h = map[&(x, y)];
    let neighbors = 
      [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
    for n in &neighbors {
      if let Some(nh) = map.get(n) {
        if (*nh == 'S' as usize || *nh == 'a' as usize)
          && h - 1 <= 'a' as usize {
          println!("{}", d + 1);
          break 'outer;
        }
        if *nh >= h - 1 && seen.insert(*n) {
          q.push_back((*n, d + 1));
        }
      }
    }
  }
}