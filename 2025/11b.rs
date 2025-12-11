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

  // Break loops: these can't lead to the goal or
  // otherwise the number of paths would diverge,
  // so the puzzle wouldn't have a solution.
  cache.insert(from, 0);

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

  let mut graph = lines
    .map(|l| {
      let mut wi = l
        .split(|c: char| !c.is_alphabetic())
        .filter(|s| s.len() > 0);

      let from = id(wi.next().expect("from"));
      let to = wi.map(|w| id(w)).collect::<Vec<_>>();
      (from, to)
    })
    .collect::<HashMap<_, _>>();

  // Make sure the out device is present with no
  // downstream connections.
  graph.insert(id("out"), Vec::new());

  // The fft and dac devices must be in one fixed
  // ordering. However, instead of determining the
  // order, summing over botb possible orders is
  // more convenient to write.
  let n = [
    [id("svr"), id("fft"), id("dac"), id("out")],
    [id("svr"), id("dac"), id("fft"), id("out")],
  ]
  .iter()
  .map(|p| {
    p.windows(2)
      .map(|w| {
        count(&graph, w[0], w[1], &mut HashMap::new())
      })
      .product::<usize>()
  })
  .sum::<usize>();

  println!("{n}");
}
