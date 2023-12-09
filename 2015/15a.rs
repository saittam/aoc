use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut c = Vec::new();
  for s in lines.take_while(|s| s.len() > 0) {
    let p = s.split(&[' ', ','][..])
             .filter_map(|s| s.parse::<i64>().ok())
             .collect::<Vec<_>>();
    c.resize(p.len(), Vec::new());
    for (ic, v) in c.iter_mut().zip(p.iter()) {
      ic.push(*v);
    }
  }
  
  c.pop();

  let mut best = 0;
  let mut a = vec![(0, 100); c[0].len()];
  (*a.last_mut().unwrap()).0 = 100;
  for _ in 0.. {
    let score = c.iter()
      .map(|ic| std::cmp::max(
                  a.iter().zip(ic.iter())
                   .map(|((i, _), c)| i * c)
                   .sum::<i64>(),
                  0))
      .product();
    best = std::cmp::max(best, score);
    
    let p = match a.iter().rev().position(|(i, l)| i < l) {
      None => break,
      Some(i) => a.len() - i - 1,
    };
    
    a[p].0 += 1;
    let rem = a[p].1 - a[p].0;
    for e in a.iter_mut().skip(p + 1) {
      *e = (0, rem);
    }
    let (i, l) = a.last_mut().unwrap();
    *i = *l;
  }
  
  println!("{}", best);
}