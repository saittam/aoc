use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let v = lines
    .take_while(|l| l.len() > 0)
    .map(|l| l.parse::<u32>().unwrap())
    .collect::<Vec<_>>();
    
  let c = v.iter().zip(v.iter().skip(1))
    .filter(|(a, b)| a < b)
    .count();
  
  println!("{}", c);
}