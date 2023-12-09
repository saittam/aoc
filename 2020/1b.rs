use std::io::BufRead;
use std::cmp::Ordering;

fn find(v: &[u32], s: u32) -> Option<u32> {
  let mut fwd = v.iter();
  let mut rev = v.iter().rev();
  
  let mut a = fwd.next().unwrap();
  let mut b = rev.next().unwrap();
  
  while a <= b {
    match (a + b).cmp(&s) {
      Ordering::Less => a = fwd.next()?,
      Ordering::Equal => return Some(a * b),
      Ordering::Greater => b = rev.next()?, 
    }
  }
  
  None
}

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
    
    v.push(buf.trim().parse::<u32>().unwrap());
  }
  
  v.sort();
  
  for i in 0..v.len() {
    let d = v[i];
    if let Some(p) = find(&v[i..], 2020 - d) {
      println!("{:?}", p * d);
      break;
    }
  }
}