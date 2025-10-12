use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let n = lines
    .map(|l| {
      let mut pi = l.split(&[':', ';']);
      let n = pi
        .next()
        .expect("game")
        .split_whitespace()
        .last()
        .expect("last")
        .parse::<usize>()
        .expect("num");
      let g = pi.map(|p| p.split(',').map(|w| {
          let mut wi = w.split_whitespace();
          let n = wi
            .next()
            .expect("n")
            .parse::<usize>()
            .expect("num");
          (wi.next().expect("color").to_string(), n)
        }).collect::<HashMap<_, _>>())
        .collect::<Vec<_>>();
      (n, g)                   
    })
    .map(|(_, g)| g.iter().fold(
      HashMap::new(),
      |mut m, d| {
        for (c, &n) in d {
          m.entry(c)
           .and_modify(|e| *e = usize::max(*e, n))
           .or_insert(n);
        }
        m
      })
      .values()
      .product::<usize>())
    .sum::<usize>();
  
  println!("{}", n);
}