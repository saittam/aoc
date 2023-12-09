use std::io::BufRead;
use std::collections::HashMap;

fn search<'a, 'b, 'c>(
    edges: &'b HashMap<&'a str, Vec<&'a str>>,
    path: &'c mut Vec<&'a str>,
    tail: &'a str) -> usize {
  if tail == "end" {
    return 1;
  }

  path.push(tail);
  let mut s = 0;
  for n in &edges[tail] {
    if n.chars().next().unwrap().is_lowercase() &&
       path.iter().find(|e| *e == n).is_some() {
      continue;
    }
    s += search(edges, path, n);
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
  let s = search(&edges, &mut path, "start");
  println!("{}", s);
}