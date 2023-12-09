use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut m = Vec::new();
  loop {  
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }
    
    let s = buf.trim().chars().map(
      |c| match c {
        'F' | 'L' => 0,
        'B' | 'R' => 1,
        _ => panic!("Bad digit: {}", c),
      }).fold(0, |a, d| (a << 1) | d);
    m.push(s);
  }
  
  println!("{}", m.iter().max().unwrap());
}