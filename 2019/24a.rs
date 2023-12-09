use std::io::BufRead;
use std::collections::HashSet;

fn get(state: usize, (x, y): (isize, isize)) -> Option<bool> {
  if x >= 0 && y >= 0 && x < 5 && y < 5 {
    Some(state & (1 << ((5 * y) + x)) > 0)
  } else {
    None
  }
}

fn show(s: usize) {
  for y in 0..5 {
    let line = (0..5).into_iter().map(
      |x| if get(s, (x, y)).unwrap() { '#' } else { '.' }
    ).collect::<String>();
    println!("{}", line);
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut state = 0;
  for y in 0.. {
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }

    let r = buf.trim().chars().enumerate().map(
      |(i, c)| match c {
        '#' => 1 << i,
        '.' => 0,
        _ => panic!("input {}", c),
      }).fold(0, |s, v| s | v);
    
    state = state | (r << (5 * y));
  }
  
  let mut seen = HashSet::new();
  loop {
    seen.insert(state);
    
    //show(state);
    //println!("");
    
    let mut newstate = 0;
    for y in 0..5 {
      for x in 0..5 {
        let neigh = [
          (x, y - 1),
          (x - 1, y),
          (x + 1, y),
          (x, y + 1),
        ];
        let s = neigh.iter().map(
          |n| get(state, *n).unwrap_or(false) as usize
        ).sum();
        let news = match (s, get(state, (x, y)).unwrap()) {
          (1, true) => true,
          (1, false) | (2, false) => true,
          _ => false,
        };
        newstate |= (news as usize) << ((5 * y) + x);
      }
    }
    
    if seen.contains(&newstate) {
      println!("{}", newstate);
      //show(newstate);
      break;
    }
    state = newstate;
  }
}