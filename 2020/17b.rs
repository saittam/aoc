use std::io::BufRead;
use std::collections::{HashSet, HashMap};

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().enumerate()
    .map(|(y, r)| (y, r.unwrap()));
  
  let mut state = HashSet::new();
  loop {  
    let (y, line) = lines.next().unwrap();
    if line.len() == 0 {
      break;
    }
    
    for (x, _) in line.chars().enumerate()
                    .filter(|(_, c)| *c == '#') {
      state.insert((x as isize, y as isize, 0isize, 0isize));
    }
  }
  
  for _ in 0..6 {
    let mut c = HashMap::new();
    for (x, y, z, w) in &state {
      let neighbors =
        ((x - 1)..(x + 2)).flat_map(
          move |nx| ((y - 1)..(y + 2)).flat_map(
            move |ny| ((z - 1)..(z + 2)).flat_map(
              move |nz| ((w - 1)..(w + 2)).map(
                move |nw| (nx, ny, nz, nw)))))
        .filter(|n| *n != (*x, *y, *z, *w));
      for n in neighbors {
        *c.entry(n).or_insert(0) += 1;
      }
    }
    
    let ns = c.iter()
      .filter(|(c, an)| **an == 3 ||
                       (**an == 2 && state.contains(&c)))
      .map(|(c, _)| *c)
      .collect::<HashSet<_>>();
    state = ns;
  }
  
  println!("{}", state.len());
}