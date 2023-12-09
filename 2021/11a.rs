use std::io::BufRead;
use std::collections::VecDeque;

fn get<'a>(m: &'a mut Vec<Vec<u8>>, p: (isize, isize))
  -> Option<&'a mut u8> {
  m.get_mut(p.1 as usize).and_then(
    |r| r.get_mut(p.0 as usize))
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut m = lines.take_while(|l| l.len() > 0).map(
    |l| l.chars()
         .map(|c| c.to_digit(10).unwrap() as u8)
         .collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let mut s = 0;
  for _ in 0..100 {
    let mut q = VecDeque::new();
    
    for (y, r) in m.iter_mut().enumerate() {
      for (x, l) in r.iter_mut().enumerate() {
        *l = (*l % 10) + 1;
        if *l == 10 {
          q.push_back((x as isize, y as isize));
        }
      }
    }
    
    while let Some((x, y)) = q.pop_front() {
      s += 1;
      let neighbors = [
        (x - 1, y - 1),
        (x    , y - 1),
        (x + 1, y - 1),
        (x - 1, y    ),
        (x + 1, y    ),
        (x - 1, y + 1),
        (x    , y + 1),
        (x + 1, y + 1),
      ];
      for n in &neighbors {
        if let Some(l) = get(&mut m, *n) {
          if *l < 10 {
            *l += 1;
            if *l == 10 {
              q.push_back(*n);
            }
          }
        }
      }
    }
  }

  println!("{}", s);
}