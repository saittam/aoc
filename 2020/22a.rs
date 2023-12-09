use std::io::BufRead;
use std::collections::VecDeque;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut decks = [ VecDeque::new(), VecDeque::new() ];
  for p in 0..2 {
    decks[p] = lines.by_ref().take_while(|l| l.len() > 0)
         .skip(1)
         .map(|l| l.parse::<u32>().unwrap())
         .collect::<VecDeque<_>>();
  }
  
  while decks.iter().all(|d| d.len() > 0) {
    let c0 = decks[0].pop_front().unwrap();
    let c1 = decks[1].pop_front().unwrap();
    
    if c0 > c1 {
      decks[0].push_back(c0);
      decks[0].push_back(c1);
    } else {
      decks[1].push_back(c1);
      decks[1].push_back(c0);
    }
  }

  let s = decks[(decks[1].len() > 0) as usize].iter().rev()
    .enumerate()
    .map(|(i, c)| (i + 1) * *c as usize)
    .sum::<usize>();
  println!("{}", s);
}