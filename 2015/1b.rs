use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut sum = 0;
  let steps = lines.next().unwrap().chars()
    .map(|c| match c {
               '(' => 1,
               ')' => -1,
               _ => panic!("bad char {}", c),
         })
    .map(|n| { sum += n; sum })
    .take_while(|n| *n >= 0)
    .count();
    
  println!("{}", steps + 1);
}
  