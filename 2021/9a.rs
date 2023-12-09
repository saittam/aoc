use std::io::BufRead;

fn get(m: &Vec<Vec<u8>>, x: isize, y: isize) -> Option<u8> {
  m.get(y as usize).and_then(|r| r.get(x as usize).cloned())
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let m = lines.take_while(|l| l.len() > 0).map(
    |l| l.chars()
         .map(|c| c.to_digit(10).unwrap() as u8)
         .collect::<Vec<_>>())
    .collect::<Vec<_>>();
  
  let mut s = 0usize;
  for (uy, r) in m.iter().enumerate() {
    for (ux, h) in r.iter().enumerate() {
      let y = uy as isize;
      let x = ux as isize;
      let n = [ (y - 1, x), (y, x - 1), (y, x + 1), (y + 1, x) ];
      if n.iter()
        .filter_map(|(ny, nx)| get(&m, *nx, *ny))
        .all(|nh| nh > *h) {
        s += *h as usize + 1;
      }
    }
  }

  println!("{}", s);
}