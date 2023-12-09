use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let floor = lines.next().unwrap().chars()
    .map(|c| match c {
               '(' => 1,
               ')' => -1,
               _ => panic!("bad char {}", c),
         })
    .sum::<isize>();
    
  println!("{}", floor);
}
  