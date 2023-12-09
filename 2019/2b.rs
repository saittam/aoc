use std::io::BufRead;

fn run(m: &mut Vec<usize>) {
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
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
    
  let mut buf = String::new();
  handle.read_line(&mut buf);
      
  let mut m = buf.split(',')
    .map(|s| s.trim().parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

  for noun in 0..100 {
    for verb in 0..100 {
      let mut s = m.clone();
      s[1] = noun;
      s[2] = verb;
      run(&mut s);
      if s[0] == 19690720 {
        println!("{}", 100 * noun + verb);
      }
    }
  }
}