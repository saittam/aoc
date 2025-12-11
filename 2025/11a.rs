use std::{collections::HashMap, io::BufRead};

fn count(
  graph: &HashMap<usize, Vec<usize>>,
  from: usize,
  goal: usize,
  cache: &mut HashMap<usize, usize>,
) -> usize {
  if from == goal {
    return 1;
  }

  if let Some(n) = cache.get(&from) {
    return *n;
  }

  let n = graph[&from]
    .iter()
    .map(|to| count(graph, *to, goal, cache))
    .sum::<usize>();
  cache.insert(from, n);
  n
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut idmap = HashMap::new();
  let mut id = |s: &str| {
    let n = idmap.len();
    *idmap.entry(s.to_string()).or_insert(n)
  };

  let graph = lines
    .map(|l| {
      let mut wi = l
        .split(|c: char| !c.is_alphabetic())
        .filter(|s| s.len() > 0);

      let from = id(wi.next().expect("from"));
      let to = wi.map(|w| id(w)).collect::<Vec<_>>();
      (from, to)
    })
    .collect::<HashMap<_, _>>();

  let n = count(
    &graph,
    id("you"),
    id("out"),
    &mut HashMap::new(),
  );

  println!("{n}");
}
