use std::io::BufRead;

const OPEN: &'static str = "<{([";
const CLOSE: &'static str = ">})]";
const SCORE: [usize; 4] = [ 25137, 1197, 3, 57 ];

fn check(s: String) -> usize {
  let mut stack = Vec::new();
  for c in s.chars() {
    if OPEN.contains(c) {
      stack.push(c);
    } else if let Some(p) = CLOSE.find(c) {
      if stack.pop() != OPEN.chars().nth(p) {
        return SCORE[p];
      }
    } else {
      panic!("Bad char {}", c);
    }
  }
  
  0
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let s: usize = lines
    .take_while(|l| l.len() > 0)
    .map(check)
    .sum();

  println!("{}", s);
}