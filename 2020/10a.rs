use std::io::BufRead;
use std::collections::HashMap;

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
    
    v.push(buf.trim().parse::<u64>().unwrap());
  }
  
  v.push(0);
  v.sort();
  let dev = v.last().unwrap() + 3;
  v.push(dev);
  let mut m = HashMap::new();
  for a in v.windows(2) {
    *m.entry(a[1] - a[0]).or_insert(0) += 1;
  }
  
  println!("{:?}", m[&1] * m[&3]);
}