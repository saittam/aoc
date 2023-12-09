use std::io::BufRead;

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
    
    v.push(buf.trim().parse::<usize>().unwrap());
  }
  
  v.sort();
  
  let mut m = vec![0; v.last().unwrap() + 1];
  m[0] = 1u64;
  for c in v {
    m[c] = m[(c.saturating_sub(3))..c].iter().sum();
  }
  
  println!("{}", m.last().unwrap());
}