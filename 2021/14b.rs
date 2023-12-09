use std::io::BufRead;
use std::collections::HashMap;
use std::iter::FromIterator;

type CH = HashMap<char, usize>;
fn merge(a: &CH, b: &CH) -> CH {
  let mut m = HashMap::new();
  for (c, n) in a.iter().chain(b.iter()) {
    *m.entry(*c).or_insert(0) += n;
  }
  m
}

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

  let mut cm = r.keys()
    .map(|(a, b)| 
      ((*a, *b), 
       HashMap::from_iter([(*a, 1)].iter().cloned())))
    .collect::<HashMap<_, _>>();
    
  for _ in 0..40 {
    cm = r.iter()
      .map(|((a, b), r)| ((*a, *b), 
                          merge(&cm[&(*a, *r)],
                                &cm[&(*r, *b)])))
      .collect::<HashMap<_, _>>();
  }
  
  let mut n = HashMap::new();
  for (a, b) in s.chars().zip(s.chars().skip(1)) {
    n = merge(&n, &cm[&(a, b)]);
  }
  *n.entry(s.chars().last().unwrap()).or_insert(0) += 1;
  
  let min = *n.values().min().unwrap();
  let max = *n.values().max().unwrap();
  println!("{}", max - min);
}
