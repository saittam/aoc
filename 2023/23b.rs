use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Pos = (isize, isize);

fn neigh((x, y): Pos) -> [Pos; 4] {
  [ (x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1) ]
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let map = lines
    .enumerate()
    .flat_map(|(y, l)| l.chars()
      .enumerate()
      .map(move |(x, c)| ((x as isize, y as isize), c))
      .collect::<Vec<_>>())
    .filter_map(|(p, c)| match c {
      '.'|'>'|'v'|'<'|'^' => Some(p),
      '#' => None,
      _ => panic!("bad tile {}", c),
    })
    .collect::<HashSet<_>>();

  let start = *map.iter().min().expect("start");
  let goal = *map.iter().max().expect("goal");

  let mut idmap = HashMap::new();
  idmap.insert(start, 0);
  idmap.insert(goal, 1);

  let mut graph = vec![[None; 4], [None; 4]];

  let mut queue = VecDeque::new();
  queue.push_back((1, start));
  while let Some((dir, node)) = queue.pop_front() {
    let nodeid = idmap[&node];
    let mut p = neigh(node)[dir];
    let mut d = dir;
    for l in 1.. {
      if p == goal {
        graph[nodeid][dir] = Some((idmap[&goal], l));
        break;
      }
      let di = (d + 2) % 4;
      let mut ni = neigh(p).into_iter().enumerate()
        .filter(|(i, _)| *i != di)
        .filter(|(_, n)| map.contains(n));
      match ni.clone().count() {
        0 => break, // dead end
        1 => (d, p) = ni.next().unwrap(), // continuation
        _ => { // junction
          let id = *idmap.entry(p).or_insert_with(|| {
            queue.extend(ni.map(|(i, _)| (i, p)));
            graph.push([None; 4]);
            graph.len() - 1
          });
          graph[nodeid][dir] = Some((id, l));
          graph[id][di] = Some((nodeid, l));
          break;
        }
      }
    }
  }

  assert!(graph.len() <= 64);

  let goali = *idmap.get(&goal).expect("goal idx");
  assert_eq!(goali, 1);

  let (max, _) = std::iter::successors(
    Some((None, HashMap::from([((0, 1u64), 0)]))),
    |(mut max, dm)| {
      let mut dmn = HashMap::new();
      for ((node, set), dist) in dm {
        let ni = graph[*node].iter()
          .filter_map(|e| *e)
          .filter(|(n, _)| (1 << n) & set == 0);
        for (n, l) in ni {
          let nset = set | (1 << n);
          let d = *dmn.entry((n, nset))
            .and_modify(|d| *d = usize::max(*d, dist + l))
            .or_insert(dist + l);
          if n == goali {
            max = Some(usize::max(max.unwrap_or(d), d));
          }
        }
      }
      if dmn.is_empty() { None } else { Some((max, dmn)) }
    })
    .last()
    .expect("last");
  let n = max.expect("max");

  println!("{}", n);
}