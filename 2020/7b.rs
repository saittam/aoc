use std::io::BufRead;
use std::collections::VecDeque;

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
  
  let mut total = 0;
  let mut q = VecDeque::new();
  q.push_back((1, ("shiny".to_string(), "gold".to_string())));
  while let Some((n, cur)) = q.pop_front() {
    total += n;
    let inner = &m.iter().find(|(c, _)| *c == cur).unwrap().1;
    for (k, c) in inner {
      q.push_back((n * k, c.clone()));
    }
  }

  println!("{:?}", total - 1);
}