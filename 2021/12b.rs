use std::io::BufRead;
use std::collections::HashMap;

fn search<'a, 'b, 'c>(
    edges: &'b HashMap<&'a str, Vec<&'a str>>,
    path: &'c mut Vec<&'a str>,
    twice: bool,
    next: &'a str) -> usize {
  if next == "end" {
    return 1;
  }

  path.push(next);
  let mut s = 0;
  for n in edges[next].iter().filter(|n| **n != "start") {
    let seen = n.chars().next().unwrap().is_lowercase() &&
               path.iter().find(|e| *e == n).is_some();
    let rtwice = match (seen, twice) {
      (true, true) => continue,
      (true, false) => true,
      _ => twice,
    };
    s += search(edges, path, rtwice, n);
  }
  path.pop();
  s
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let input = lines.take_while(|l| l.len() > 0)
                   .collect::<Vec<_>>();
  let mut edges = HashMap::new();
  for l in &input {
    let mut i = l.split('-');
    let a = i.next().unwrap();
    let b = i.next().unwrap();
    
    edges.entry(a).or_insert_with(|| Vec::new()).push(b);
    edges.entry(b).or_insert_with(|| Vec::new()).push(a);
  }
  
  let mut path = Vec::new();
  let s = search(&edges, &mut path, false, "start");
  println!("{}", s);
}