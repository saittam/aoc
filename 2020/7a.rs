use std::io::BufRead;
use std::collections::{HashSet, VecDeque};

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut m = Vec::new();
  loop {  
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }
    
    let mut c = buf.trim().split(char::is_whitespace);
    let outer = (c.next().unwrap().to_string(),
                 c.next().unwrap().to_string());
    c.next();
    c.next();
    let mut inner = Vec::new();
    loop {
      if let Some(ns) = c.next() {
        if ns == "no" {
          break;
        }
        inner.push((ns.parse::<u32>().unwrap(),
                    (c.next().unwrap().to_string(),
                     c.next().unwrap().to_string())));
        c.next();
      } else {
        break;
      }
    }
    
    m.push((outer, inner));
  }
  
  let mut s = HashSet::new();
  let mut q = VecDeque::new();
  q.push_back(("shiny".to_string(), "gold".to_string()));
  while let Some(cur) = q.pop_front() {
    for (outer, inner) in &m {
      if inner.iter().filter(|(_, c)| *c == cur).next().is_some() &&
         !s.contains(&*outer) {
        s.insert(outer.clone());
        q.push_back(outer.clone());
      }
    }
  }

  println!("{:?}", s.len());
}