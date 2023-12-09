use std::io::BufRead;
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug)]
enum Insn {
  Nop,
  Acc,
  Jmp,
}

impl TryFrom<&str> for Insn {
  type Error = &'static str;
  
  fn try_from(s: &str) -> Result<Insn, Self::Error> {
    match s {
      "nop" => Ok(Insn::Nop),
      "acc" => Ok(Insn::Acc),
      "jmp" => Ok(Insn::Jmp),
      _ => Err("bad instruction"),
    }
  }
}

#[derive(Debug)]
enum Termination {
  Loop,
  Halt,
}

fn run(p: &[(Insn, i64)]) -> (Termination, i64) {
  let mut acc = 0;
  let mut ip = 0;
  
  let mut pos = vec![false; p.len()];
  
  loop {
    let (i, a) = match p.get(ip) {
      Some(ia) => ia,
      None => return (Termination::Halt, acc),
    };
    
    if pos[ip] == true {
      return (Termination::Loop, acc);
    }
    pos[ip] = true;
    ip += 1;
    
    match i {
      Insn::Nop => (),
      Insn::Acc => acc += a,
      Insn::Jmp => ip = (ip as i64 + a - 1) as usize,
    }
  }
}

fn main() -> Result<(), Box<std::error::Error>> {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut m = Vec::new();
  loop {  
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }
    
    let mut s = buf.trim().split(char::is_whitespace);
    let insn = Insn::try_from(s.next().ok_or("no insn")?)?;
    let arg = s.next().ok_or("no arg")?.parse::<i64>()?;
    m.push((insn, arg));
  }
  
  for i in 0..m.len() {
    let b = m[i].0;
    let subs = match b {
      Insn::Nop => Insn::Jmp,
      Insn::Acc => continue,
      Insn::Jmp => Insn::Nop,
    };
    m[i].0 = subs;
    if let (Termination::Halt, v) = run(&m) {
      println!("{:?}", v);
    }
    m[i].0 = b;
  }
  
  Ok(())
}