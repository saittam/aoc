use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut state = lines.take(2)
    .map(|l| (l.split(':').skip(1).next().unwrap()
               .trim().parse::<usize>().unwrap(),
              0))
    .collect::<Vec<_>>();
    
  let mut dice = (1..=100).cycle().step_by(3)
    .map(|n| 3 * n + 3);
  for (k, m) in dice.enumerate() {
    let (p, s) = state[k % 2];
    let np = (p + m - 1) % 10 + 1;
    let ns = s + np;
    if ns >= 1000 {
      println!("{}", state[1 - k % 2].1 * (k + 1) * 3);
      break;
    }
    state[k % 2] = (np, ns);
  }
}