use std::io::BufRead;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::cmp::Ordering;

struct Diff<E: Ord,
            A: Iterator<Item=E>,
            B: Iterator<Item=E>> {
  ai: A,
  bi: B,
}

impl<E, A, B> Iterator for Diff<E, A, B>
where
  E: Ord,
  A: Iterator<Item=E>,
  B: Iterator<Item=E>
{
  type Item = E;

  fn next(&mut self) -> Option<E> {
    let mut a = self.ai.next()?;
    let mut b = self.bi.next()?;
    loop {
      match a.cmp(&b) {
        Ordering::Less => a = self.ai.next()?,
        Ordering::Equal => return Some(a),
        Ordering::Greater => b = self.bi.next()?,
      }
    }
  }
}

fn diff<E, A, B>(a: A, b: B) -> Diff<E, A, B> 
where
  E: Ord,
  A: Iterator<Item=E>,
  B: Iterator<Item=E>
{
  Diff { ai: a, bi: b }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut v = Vec::new();
  for line in lines.by_ref().take_while(|l| l.len() > 0) {
    let mut ia = line.split("(contains")
                     .map(|s| s.to_string());
    let is = ia.next().unwrap();
    let iv = is.trim().split(' ').map(|s| s.to_string())
      .collect::<Vec<_>>();
    let av = ia.next().unwrap().trim().split(' ')
      .map(|a| a.chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .collect::<String>())
      .collect::<Vec<_>>();
    v.push((iv, av));
  }
  
  let mut alg = HashMap::new();
  for (iv, av) in &v {
    let mut ivs = iv.iter().collect::<Vec<_>>();
    ivs.sort();
    for name in av {
      match alg.entry(name) {
        Entry::Vacant(mut e) => {
          e.insert(ivs.clone());
        }
        Entry::Occupied(mut e) => {
          let d = diff(e.get().iter().map(|i| *i),
                       ivs.iter().map(|i| *i))
                   .collect::<Vec<_>>();
          e.insert(d);
        }
      }
    }
  }
  
  let mut m = HashMap::new();
  while alg.len() > 0 {
    let mut q = Vec::new();
    for (a, iv) in &alg {
      if iv.len() == 1 {
        q.push((iv.first().unwrap().clone(), a.clone()));
      }
    }
    
    assert!(q.len() > 0);
    
    for (i, a) in q {
      alg.remove(a);
      for iv in alg.values_mut() {
        iv.retain(|ie| *ie != i);
      }
      assert!(m.insert(i, a).is_none());
    }
  }
  
  let s = v.iter()
    .map(|(iv, _)| iv.iter()
           .filter(|i| !m.contains_key(i)).count())
    .sum::<usize>();
  println!("{}", s);
}