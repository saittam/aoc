use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  let mut h = std::collections::HashMap::<String, String>::new();
    
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
    
  let mut total = 0;
  
  for mut b in h.keys() {
    loop {
      match h.get(b) {
        Some(c) => {
          total += 1;
          b = c;
        }
        None => break
      }
    }
  }
    
  println!("{}", total);
}