use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut buf = String::new();
  handle.read_line(&mut buf);
  let mut v = buf.trim().split(',')
    .map(|s| s.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();
    
  while v.len() < 2020 {
    let n = *v.last().unwrap();
    let m = v.iter().rev().skip(1)
      .position(|e| *e == n)
      .map(|d| d + 1)
      .unwrap_or(0);
    v.push(m);
  }

  println!("{}", v.last().unwrap());
}