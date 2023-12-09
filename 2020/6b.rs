use std::io::BufRead;
use std::ops::BitAnd;

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut m = Vec::new();
  loop {
    let mut d = Vec::new();
    loop {
      let mut buf = String::new();
      handle.read_line(&mut buf);
    
      if buf.trim().len() == 0 {
        break;
      }
      d.push(buf.trim().chars()
        .fold(0, |a, c| a | (1 << (c as u32 - 'a' as u32))));
    }
    
    if d.len() == 0 {
      break;
    }
    
    m.push(d);
  }
  
  let s : u32 = m.iter()
    .map(|l| l.iter()
              .fold(0xffffffff, u32::bitand)
              .count_ones())
    .sum();
  
  println!("{:?}", s);
}