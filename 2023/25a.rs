use std::io::BufRead;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn edge(n1: usize, n2: usize) -> (usize, usize) {
  (usize::min(n1, n2), usize::max(n1, n2))
}

fn path(neigh: &HashMap<usize, Vec<usize>>,
        inout: &HashMap<usize, bool>,
        start: usize,
        edges_used: &mut HashSet<(usize, usize)>) -> bool {
  assert!(!inout.contains_key(&start));

  let mut queue = VecDeque::new();
  queue.push_back(start);
  let mut links = HashMap::new();
  links.insert(start, start);
  while let Some(node) = queue.pop_front() {
    if inout.get(&node) == Some(&true) {
      let mut n = node;
      while n != start {
        let prev = links[&n];
        edges_used.insert(edge(prev, n));
        n = prev;
      }
      return true;
    }

    queue.extend(neigh[&node].iter()
      .filter(|n| !edges_used.contains(&edge(node, **n)))
      .filter(|n| match links.entry(**n) {
        Entry::Occupied(_) => false,
        Entry::Vacant(e) => {
          e.insert(node);
          true
        }
      }));
  }

  false
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut idmap = HashMap::new();
  let mut getid = |s: &str| {
    let nextid = idmap.len();
    *idmap.entry(s.to_owned()).or_insert(nextid)
  };

  let neigh = lines
    .fold(HashMap::new(), |neigh, l| {
      let mut ids = l
        .split(|c: char| !c.is_alphabetic())
        .filter(|n| !n.is_empty())
        .map(|n| getid(n));

      let left = ids.next().expect("left");
      ids.fold(neigh, |mut neigh, right| {
        neigh.entry(left).or_insert_with(Vec::new).push(right);
        neigh.entry(right).or_insert_with(Vec::new).push(left);
        neigh
      })
    });

  let mut inout = HashMap::new();
  inout.insert(0, true);
  for node in 1..neigh.len() {
    let mut edges_used = HashSet::new();
    let four_reachable = (0..4)
      .all(|_| path(&neigh, &inout, node, &mut edges_used));
    inout.insert(node, four_reachable);
  }

  let n = inout.values().filter(|i| **i).count();
  println!("{}", n * (neigh.len() - n));
}