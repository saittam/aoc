use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut s = lines.by_ref().next().unwrap();
  let mut r = lines.skip(1).take_while(|l| l.len() > 0)
    .map(|l| { 
      let mut i = l.split(" -> ");
      let mut p = i.next().unwrap().chars();
      ((p.next().unwrap(), p.next().unwrap()),
       i.next().unwrap().chars().next().unwrap())
    })
    .collect::<HashMap<_, _>>();
    
  for _ in 0..10 {
    let mut ns = String::new();
    for (a, b) in s.chars().zip(s.chars().skip(1)) {
      ns.push(a);
      ns.push(r[&(a, b)]);
    }
    ns.push(s.chars().last().unwrap());
    s = ns;
  }
  
  let mut n = HashMap::new();
  for c in s.chars() {
    *n.entry(c).or_insert(0) += 1;
  }
  
  let min = *n.values().min().unwrap();
  let max = *n.values().max().unwrap();
  println!("{}", max - min);
}
