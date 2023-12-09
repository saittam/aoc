use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let start = lines.take(2)
    .map(|l| l.split(':').skip(1).next().unwrap()
              .trim().parse::<usize>().unwrap())
    .collect::<Vec<_>>();

  let mut wins = [0; 2];
  let mut state = [[0; 10 * 21]; 2];
  for (i, s) in start.iter().enumerate() {
    state[i][*s - 1] = 1;
  }
  for m in 0.. {
    let c = state[m % 2];
    let mut nc = [0; 10 * 21];
    for (k, n) in c.iter()
                   .enumerate()
                   .filter(|(_, n)| **n > 0) {
      let s = k / 10;
      let p = k % 10;
      
      for i in (1..=3).flat_map(move |i1|
               (1..=3).flat_map(move |i2|
               (1..=3).map(move |i3| i1 + i2 + i3))) {
        let np = (p + i) % 10;
        let ns = s + np + 1;
        
        if ns >= 21 {
          wins[m % 2] +=
            n * state[1 - m % 2].iter().sum::<usize>();
        } else {
          nc[ns * 10 + np] += n;
        }
      }
    }
    if nc.iter().all(|n| *n == 0) {
      break;
    }
    state[m % 2] = nc;
  }
  
  println!("{:?}", wins.iter().max().unwrap());
}