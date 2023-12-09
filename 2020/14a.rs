use std::io::BufRead;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Insn {
  Mask(u64, u64),
  Store(u64, u64),
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
    
    let mut p = buf.trim().split('=');
    let w = p.next().unwrap().trim();
    let insn = if w == "mask" {
      let m = p.next().unwrap().trim();
      let s = m.chars()
        .fold(0, |a, c| (a << 1) | (c == '1') as u64);
      let c = m.chars()
        .fold(0, |a, c| (a << 1) | (c != '0') as u64);
      Insn::Mask(s, c)
    } else if w.starts_with("mem[") {
      let addr = w.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u64>().unwrap();
      let val = p.next().unwrap().trim()
        .parse::<u64>().unwrap();
      Insn::Store(addr, val)
    } else {
      panic!("bad instruction {}", buf.trim());
    };
    v.push(insn);
  }
  
  let mut mem = HashMap::new();
  let mut mask = (0, !0);
  for insn in v {
    match insn {
      Insn::Mask(s, c) => mask = (s, c),
      Insn::Store(a, v) => {
        mem.insert(a, (v & mask.1) | mask.0);
        ()
      },
    }
  }
  
  let s = mem.values().sum::<u64>();
  println!("{}", s);
}