use std::io::BufRead;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Loc {
  Empty,
  East,
  South,
}

impl Loc {
  fn parse(c: char) -> Option<Loc> {
    Some(match c {
      '.' => Loc::Empty,
      '>' => Loc::East,
      'v' => Loc::South,
      _ => return None,
    })
  }
}

use Loc::{Empty, East, South};

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut m = lines.take_while(|l| l.len() > 0)
    .map(|l| l.chars()
              .map(|c| Loc::parse(c).expect("bad char"))
              .collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let pred = |k, n| (k + n - 1) % n;
  let succ = |k, n| (k + 1) % n;
  let h = m.len();
  let w = m[0].len();
  for n in 1.. {
    let me = m.iter().enumerate()
      .map(|(y, r)| r.iter().enumerate()
        .map(|(x, l)| match l {
            Empty if m[y][pred(x, w)] == East => East,
            East if m[y][succ(x, w)] == Empty => Empty,
            _ => *l,
          })
        .collect::<Vec<_>>())
      .collect::<Vec<_>>();
      
    let ms = me.iter().enumerate()
      .map(|(y, r)| r.iter().enumerate()
        .map(|(x, l)| match l {
            Empty if me[pred(y, h)][x] == South => South,
            South if me[succ(y, h)][x] == Empty => Empty,
            _ => *l,
          })
        .collect::<Vec<_>>())
      .collect::<Vec<_>>();

    if ms == m {
      println!("{}", n);
      break;
    }
    m = ms;
  }
}