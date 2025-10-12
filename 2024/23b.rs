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

  let mut cliques = graph.keys()
    .map(|n| BTreeSet::from([*n]))
    .collect::<BTreeSet<_>>();
  loop {
    let new_cliques = cliques.iter().flat_map(
      |clique| graph.iter().filter_map(
        |(n, nn)| clique.is_subset(nn).then(|| {
          let mut clique = clique.clone();
          clique.insert(n);
          clique
        })))
      .collect::<BTreeSet<_>>();
    if new_cliques.is_empty() {
      break;
    }
    cliques = new_cliques;
  }
  
  let mut nodes = cliques.first().unwrap().iter()
    .copied()
    .collect::<Vec<_>>();
  nodes.sort();
  let pwd = nodes.join(",");
              
  println!("{pwd}");
}