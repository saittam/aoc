use std::io::BufRead;
use std::ops::BitAnd;
use std::ops::Not;
use std::ops::Index;

#[derive(Clone, Debug)]
struct BitSet(Vec<bool>);

impl BitSet {
  fn new() -> BitSet {
    BitSet(Vec::new())
  }
  
  fn len(&self) -> usize {
    self.0.len()
  }
  
  fn push(&mut self, b: bool) {
    self.0.push(b);
  }
  
  fn resize(&mut self, n: usize, b: bool) {
    self.0.resize(n, b);
  }
  
  fn count_ones(&self) -> usize {
    self.0.iter().filter(|b| **b).count()
  }
  
  fn leading_zeros(&self) -> usize {
    self.0.iter().take_while(|b| !**b).count()
  }
}

impl BitAnd for &BitSet {
  type Output = BitSet;
  
  fn bitand(self, rhs: &BitSet) -> BitSet {
    BitSet(self.0.iter().zip(rhs.0.iter())
      .map(|(a, b)| *a && *b)
      .collect())
  }
}

impl Not for BitSet {
  type Output = BitSet;
  
  fn not(self) -> BitSet {
    BitSet(self.0.iter().map(|b| !*b).collect())
  }
}

impl Index<usize> for BitSet {
  type Output = bool;
  
  fn index(&self, i: usize) -> &bool {
    &self.0[i]
  }
}

fn find(n: usize, p: &[BitSet], pol: bool) -> u32 {
  let mut mask = BitSet::new();
  mask.resize(n, true);
  
  for c in p {
    let mc = mask.count_ones();
    if mc <= 1 {
      break;
    }
    
    let v = c & &mask;
    if (2 * v.count_ones() >= mc) ^ pol {
      mask = &mask & &v;
    } else {
      mask = &mask & &!v;
    }
  }
  
  let i = mask.leading_zeros();
  p.iter().fold(0, |v, c| (v << 1) | (c[i] as u32))
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut n = 0;
  let mut p = Vec::new();
  for l in lines.take_while(|l| l.len() > 0) {
    for (i, c) in l.chars().enumerate() {
      if p.len() <= i {
        p.resize(i + 1, BitSet::new());
      }
      p[i].push(c == '1');
    }
    n += 1;
  }
  
  println!("{}", find(n, &p, true) * find(n, &p, false));
}