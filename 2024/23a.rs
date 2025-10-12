use std::io::BufRead;
use std::collections::{HashMap, BTreeSet};

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let lines = lines.collect::<Vec<_>>();

  let graph = lines.iter()
    .map(|l| {
      let mut ni = l.split('-');
      (ni.next().expect("a"), ni.next().expect("b"))
    })
    .fold(HashMap::new(),
          |mut graph, (a, b)| {
            graph.entry(a).or_insert_with(BTreeSet::new)
            .insert(b);
            graph.entry(b).or_insert_with(BTreeSet::new)
            .insert(a);
            graph
          });

  let n = graph.iter()
    .flat_map(|(a, an)| an.iter()
              .flat_map(|b| an.intersection(&graph[b])
                        .map(|c| (*a, *b, *c))))
    .filter(
      |(a, b, c)| a < b && b < c &&
      [a, b, c].into_iter().any(|n| n.starts_with("t")))
    .count();
              
  println!("{n}");
}