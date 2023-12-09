use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let input = lines.next().unwrap().chars()
                   .map(|c| c.to_digit(10).unwrap() as u32)
                   .collect::<Vec<_>>();
  let maxi = *input.iter().max().unwrap();
  let mini = *input.iter().min().unwrap();
  
  const LEN: usize = 1000000;
  let c = input.iter().cloned().chain((maxi + 1)..).take(LEN);
  let mut next = vec![0u32; LEN - input.len() + maxi as usize + 1];
  let maxc = next.len() as u32 - 1;
  let minc = mini;

  let mut p = 0;
  for e in c {
    next[p] = e;
    p = e as usize;
  }
  next[p] = input[0];

  let mut cur = input[0];
  let mut _n = 0;
  for _i in 0..10000000 {
    let mut triple = [0; 3];
    let mut p = cur;
    for i in 0..3 {
      p = next[p as usize];
      triple[i] = p;
    }
    next[cur as usize] = next[p as usize];
    
    let mut target = cur;
    loop {
      target = if target == minc { maxc } else { target - 1 };
      if !triple.iter().any(|e| *e == target) {
        break;
      }
    }
    
    cur = next[triple[2] as usize];
    next[triple[2] as usize] = next[target as usize];
    next[target as usize] = triple[0];
  }

  let c1 = next[1] as usize;
  let c2 = next[c1] as usize;
  println!("{}", c1 * c2);
}