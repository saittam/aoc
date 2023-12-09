use std::io::BufRead;
use std::str::FromStr;

enum Move {
  Forward,
  Up,
  Down,
}

impl FromStr for Move {
  type Err = ;
  fn from_str(s: &str) -> Result<Move, ()> {
    match s {
      "forward" => Ok(Move::Forward),
      "up" => Ok(Move::Up),
      "down" => Ok(Move:: Down),
      _ => Err(()),
    }
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let (x, z) = lines
    .take_while(|l| l.len() > 0)
    .map(|l| {
      let mut t = l.split(' ');
      (t.next().unwrap().parse::<Move>().unwrap(),
       t.next().unwrap().parse::<u32>().unwrap())
    })
    .fold((0, 0), |(x, z), (m, d)| match m {
      Move::Forward => (x + d, z),
      Move::Up => (x, z - d),
      Move::Down => (x, z + d),
    });
  
  println!("{}", x * z);
}