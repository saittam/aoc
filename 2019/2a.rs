use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
    
  let mut buf = String::new();
  handle.read_line(&mut buf);
      
  let mut m = buf.split(',')
    .map(|s| s.trim().parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

  m[1] = 12;
  m[2] = 2;
  
  let mut ip = 0;
  loop {
    match m[ip] {
      1 => {
        let da = m[ip + 3];
        m[da] = m[m[ip + 1]] + m[m[ip + 2]];
      }
      2 => {
        let da = m[ip + 3];
        m[da] = m[m[ip + 1]] * m[m[ip + 2]];
      }
      99 => break,
      _ => panic!("invalid opcode {}", m[ip]),
    }
    
    ip += 4;
  }
  
  println!("{}", m[0]);
}