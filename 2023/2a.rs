use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let max: HashMap<&'static str, usize> = [
    ("red", 12),
    ("green", 13),
    ("blue", 14),
  ].into_iter().collect();
  
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
    .filter(|(_, g)| g.iter().all(
      |d| d.iter().all(
        |(c, n)| max.get(&c.as_str())
                    .filter(|m| n <= m)
                    .is_some())))
    .map(|(k, _)| k)
    .sum::<usize>();
  
  println!("{}", n);
}