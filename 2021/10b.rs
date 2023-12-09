use std::io::BufRead;

const OPEN: &'static str = "<{([";
const CLOSE: &'static str = ">})]";
const SCORE: [usize; 4] = [ 4, 3, 1, 2 ];

fn check(s: String) -> Option<usize> {
  let mut stack = Vec::new();
  for c in s.chars() {
    if OPEN.contains(c) {
      stack.push(c);
    } else if let Some(p) = CLOSE.find(c) {
      if stack.pop() != OPEN.chars().nth(p) {
        return None;
      }
    } else {
      panic!("Bad char {}", c);
    }
  }
  
  Some(stack.iter().rev()
    .fold(0, |s, c| s * 5 + SCORE[OPEN.find(*c).unwrap()]))
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut s = lines
    .take_while(|l| l.len() > 0)
    .filter_map(check)
    .collect::<Vec<_>>();
  s.sort();

  println!("{}", s[s.len() / 2]);
}