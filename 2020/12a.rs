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

  let mut pos = (0, 0);
  let mut dir = 0;
  let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
  
  for m in v {
    match m {
      Move::Trans(x, y) => pos = (pos.0 + x, pos.1 + y),
      Move::Rot(n) => dir = (dir + n as usize) % dirs.len(),
      Move::Fwd(n) => {
        let d = dirs[dir];
        pos = (pos.0 + n * d.0, pos.1 + n * d.1);
      },
    }
  }

  println!("{}", pos.0.abs() + pos.1.abs());
}