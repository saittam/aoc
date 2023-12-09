use std::io::BufRead;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Move {
  Trans(isize, isize),
  Rot(isize),
  Fwd(isize),
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut v = Vec::new();
  loop {  
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }
    
    let (ms, ns) = buf.trim().split_at(1);
    let n = ns.parse::<isize>().unwrap();
    let m = match ms {
      "E" => Move::Trans(n, 0),
      "S" => Move::Trans(0, n),
      "W" => Move::Trans(-n, 0),
      "N" => Move::Trans(0, -n),
      "L" => Move::Rot((360 - n) / 90),
      "R" => Move::Rot(n / 90),
      "F" => Move::Fwd(n),
      _ => panic!("Bad move {}", buf),
    };
    v.push(m);
  }

  let mut vec = (10, -1);
  let mut pos = (0, 0);
  let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
  
  for m in v {
    match m {
      Move::Trans(x, y) =>
        vec = (vec.0 + x, vec.1 + y),
      Move::Rot(n) =>
        vec = (0..n).fold(vec, |(x, y), _| (-y, x)),
      Move::Fwd(n) =>
        pos = (pos.0 + n * vec.0, pos.1 + n * vec.1),
    }
  }

  println!("{}", pos.0.abs() + pos.1.abs());
}