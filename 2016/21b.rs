use std::io::BufRead;

#[derive(Debug)]
enum Op {
  SwapPos(usize, usize),
  SwapLetter(char, char),
  RotateLeft(usize),
  RotateRight(usize),
  RotateLetter(char),
  Reverse(usize, usize),
  Move(usize, usize),
}

impl Op {
  fn parse(l: &str) -> Option<Op> {
    let mut wi = l.split_whitespace();
    let mut nums = l.split_whitespace()
      .filter_map(|w| w.parse::<usize>().ok());
    let mut letters = l.split_whitespace()
      .filter_map(|w| {
        let mut ci = w.chars();
        ci.next().xor(ci.next())
      });
    Some(match (wi.next()?, wi.next()?) {
      ("swap", "position") =>
        Op::SwapPos(nums.next()?, nums.next()?),
      ("swap", "letter") =>
        Op::SwapLetter(letters.next()?, letters.next()?),
      ("rotate", "left") => Op::RotateLeft(nums.next()?),
      ("rotate", "right") => Op::RotateRight(nums.next()?),
      ("rotate", "based") =>
        Op::RotateLetter(letters.next()?),
      ("reverse", _) =>
        Op::Reverse(nums.next()?, nums.next()?),
      ("move", _) => Op::Move(nums.next()?, nums.next()?),
      _ => return None,
    })
  }

  #[allow(dead_code)]
  fn transform(&self, s: &mut [char]) {
    match self {
      Op::SwapPos(x, y) => s.swap(*x, *y),
      Op::SwapLetter(a, b) => {
        let ai = s.iter().position(|c| *c == *a).expect("ai");
        let bi = s.iter().position(|c| *c == *b).expect("bi");
        s.swap(ai, bi);
      }
      Op::RotateLeft(k) => s.rotate_left(*k),
      Op::RotateRight(k) => s.rotate_right(*k),
      Op::RotateLetter(a) => {
        let i = s.iter().position(|c| *c == *a).expect("i");
        let k = 1 + i + if i >= 4 { 1 } else { 0 };
        s.rotate_right(k % s.len());
      }
      Op::Reverse(x, y) => s[*x..=*y].reverse(),
      Op::Move(x, y) => if *x <= *y {
        s[*x..=*y].rotate_left(1);
      } else {
        s[*y..=*x].rotate_right(1);
      }
    }
  }

  fn reverse(&self, s: &mut [char]) {
    match self {
      Op::SwapPos(x, y) => s.swap(*x, *y),
      Op::SwapLetter(a, b) => {
        let ai = s.iter().position(|c| *c == *a).expect("ai");
        let bi = s.iter().position(|c| *c == *b).expect("bi");
        s.swap(ai, bi);
      }
      Op::RotateLeft(k) => s.rotate_right(*k),
      Op::RotateRight(k) => s.rotate_left(*k),
      /*
      0 -> 1
      1 -> 3
      2 -> 5
      3 -> 7
      4 -> 2
      5 -> 4
      6 -> 6
      7 -> 0
      */
      Op::RotateLetter(a) => {
        let i = s.iter().position(|c| *c == *a).expect("i");
        s.rotate_left([1, 1, 6, 2, 7, 3, 0, 4][i]);
      }
      Op::Reverse(x, y) => s[*x..=*y].reverse(),
      Op::Move(x, y) => if *x <= *y {
        s[*x..=*y].rotate_right(1);
      } else {
        s[*y..=*x].rotate_left(1);
      }
    }
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let ops = lines
    .map(|l| Op::parse(&l).expect("parse"))
    .collect::<Vec<_>>();

  let mut s = "fbgdceah".chars().collect::<Vec<_>>();

  for o in ops.iter().rev() {
    o.reverse(&mut s);
  }
    
  println!("{}", s.iter().collect::<String>());
}