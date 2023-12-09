use std::io::BufRead;
use std::collections::HashMap;

fn path<'a>(h: &'a HashMap::<String, String>, s: &str) -> Vec<&'a String> {
  let mut p = Vec::new(); 
  let mut b = s;
  loop {
    match h.get(b) {
      Some(c) => {
        p.push(c);
        b = c;
      }
      None => break
    }
  }
  
  return p;
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  let mut h = HashMap::<String, String>::new();
    
  loop {
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }
    //println!("buf: {}", buf);
    let mut ci = buf.split(')');
    let c = ci.next().unwrap().trim().to_string();
    let b = ci.next().unwrap().trim().to_string();
    h.insert(b, c);
  }
    
  let pyou = path(&h, &"YOU");
  let psan = path(&h, &"SAN");
  //println!("{:?} {:?}", pyou, psan);
  
  let mut common = 0;
  for (a, b) in pyou.iter().rev().zip(psan.iter().rev()) {
    //println!("{:?} {:?}", a, b);
    if a != b {
      break;
    }
    common += 1;
  }
    
  println!("{}", pyou.len() + psan.len() - 2 * common);
}