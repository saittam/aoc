use std::io::BufRead;
use std::collections::VecDeque;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let passcode = lines.next().expect("salt");

  let mut queue = VecDeque::new();
  queue.push_back(((1, 1), passcode.to_string()));
  while let Some(((x, y), pc)) = queue.pop_front() {
    if (x, y) == (4, 4) {
      println!("{}", pc.strip_prefix(&passcode).unwrap());
      break;
    }

    let neigh = [
      ((x, y - 1), 'U'),
      ((x, y + 1), 'D'),
      ((x - 1, y), 'L'),
      ((x + 1, y), 'R'),
    ];
    let hash = md5::compute(&pc);
    let openi = hash.iter()
      .flat_map(|b| [b >> 4, b & 0xf])
      .map(|b| b >= 0xb);
    for (_, (n, d)) in openi.zip(neigh)
      .filter(|(o, _)| *o)
      .filter(|(_, ((x, y), _))| 
              *x > 0 && *y > 0 && *x < 5 && *y < 5) {
      queue.push_back((n, format!("{}{}", pc, d)));
    }
  }
}