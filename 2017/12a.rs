use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let pipes = lines.map(|l| {
      let mut ni = l.split(|c: char| !c.is_numeric())
        .filter_map(|w| w.parse::<u32>().ok());
      (ni.next().expect("n"), ni.collect::<Vec<_>>())
    })
    .collect::<HashMap<_, _>>();

  let mut queue = VecDeque::new();
  queue.push_back(0);
  let mut seen = HashSet::new();
  while let Some(p) = queue.pop_front() {
    if !seen.insert(p) {
      continue;
    }
    queue.extend(&pipes[&p]);
  }
  
  println!("{}", seen.len());
}