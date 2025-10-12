use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let turns = lines.next().expect("turns").chars()
    .map(|c| match c {
      'L' => 0,
      'R' => 1,
      _ => panic!("bad turn"),
    })
    .collect::<Vec<_>>();

  let graph = lines.skip(1)
    .map(|l| {
      let mut ni = l.split(|c: char| !c.is_alphabetic())
        .filter(|w| !w.is_empty())
        .map(str::to_owned);
      (ni.next().expect("node"), ni.collect::<Vec<_>>())
    })
    .collect::<HashMap<_, _>>();
  
  let n = 1 + turns.iter().cycle().scan("AAA", |pos, t| {
      *pos = graph.get(*pos)?.get(*t)?;
      Some(*pos)
    })
    .position(|p| p == "ZZZ")
    .expect("no solution");
 
  println!("{}", n);
}