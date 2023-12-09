use std::io::BufRead;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut houses = HashSet::new();
  lines.next().unwrap().chars()
    .map(|c| match c {
               '^' => (0, -1),
               'v' => (0, 1),
               '>' => (1, 0),
               '<' => (-1, 0),
               _ => panic!("bad direction {}", c),
             })
    .fold((0, 0), 
          |(px, py), (dx, dy)| {
            let r = (px + dx, py + dy);
            houses.insert(r);
            r
          });

  println!("{}", houses.len());
}