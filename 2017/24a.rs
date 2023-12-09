use std::io::BufRead;
use std::collections::HashMap;

fn search(edges: &HashMap<u32, Vec<(u64, u32)>>,
          pos: u32,
          used: u64,
          strength: u32) -> u32 {
  edges[&pos].iter()
    .filter(|(i, _)| used & (1 << i) == 0)
    .map(|(i, next)| search(edges, *next, used | (1 << i),
                            strength + pos + next))
    .max()
    .unwrap_or(strength)
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let edges = lines
    .map(|l| {
      let mut ni = l
        .split('/')
        .map(|w| w.parse::<u32>().expect("num"));
      (ni.next().expect("left"), ni.next().expect("right"))
    })
    .enumerate()
    .fold(HashMap::new(), |mut m, (i, (l, r))| {
      m.entry(l).or_insert_with(Vec::new).push((i as u64, r));
      m.entry(r).or_insert_with(Vec::new).push((i as u64, l));
      m
    });

  let best = search(&edges, 0, 0, 0);

  println!("{}", best);
}
