use std::io::BufRead;

type Val = u64;

struct VM<'a> {
  prog: &'a [Val],
  ip: Val,
  regs: [Val; 3],
}

impl<'a> VM<'a> {

  fn combo(&self, arg: Val) -> Val {
    match arg {
      0..=3 => arg,
      4..=6 => self.regs[arg as usize - 4],
      _ => panic!("bad combo {arg}"),
    }
  }
  
  fn step(&mut self) -> Option<Option<Val>> {
    let insn = *self.prog.get(self.ip as usize)?;
    let arg = *self.prog.get(self.ip as usize + 1)?;
    self.ip += 2;

    let mut out = None;
    match insn {
      0 => self.regs[0] >>= self.combo(arg),
      1 => self.regs[1] ^= arg,
      2 => self.regs[1] = self.combo(arg) % 8,
      3 => if self.regs[0] != 0 { self.ip = arg },
      4 => self.regs[1] ^= self.regs[2],
      5 => out = Some(self.combo(arg) % 8),
      6 => self.regs[1] = self.regs[0] >> self.combo(arg),
      7 => self.regs[2] = self.regs[0] >> self.combo(arg),
      _ => panic!("bad instruction {insn}"),
    }
    Some(out)
  }
}

fn find(v: Val, goal: &[Val]) -> Option<Val> {
  match goal {
    [] => Some(v),
    [n, ..] => (0..8).filter_map(|k| {
      let v = (v << 3) | k;
      let t = (v % 8) ^ 1;
      if ((v >> t) ^ (t ^ 5)) % 8 == *n {
        let r = find(v, &goal[1..]);
        if r == None {
          println!("backtrack");
        }
        r
      } else {
        None
      }
    }).next()
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let regs = lines.by_ref()
    .take_while(|l| !l.is_empty())
    .map(|l| l.split(|c: char| !c.is_digit(10))
         .filter(|w| !w.is_empty())
         .map(|w| w.parse::<Val>().expect("num"))
         .next()
         .expect("empty line"))
    .collect::<Vec<_>>();
  let mut regs: [Val; 3] = regs.try_into().expect("regs");

  let prog = lines.next().expect("prog")
    .split(|c: char| !c.is_digit(10))
    .filter(|w| !w.is_empty())
    .map(|w| w.parse::<Val>().expect("num"))
    .collect::<Vec<_>>();

  let mut rprog = prog.clone();
  rprog.reverse();
  let a = find(0, &rprog[0..]).expect("no solution");
  regs[0] = a;
  println!("{:#x}", regs[0]);
  let mut vm = VM {
    prog: &prog,
    ip: 0,
    regs,
  };
  let out = std::iter::from_fn(|| vm.step())
    .filter_map(|o| o)
    .collect::<Vec<_>>();
  println!("{out:?}");
  println!("{prog:?}");
  println!("{a}");

  /*
  let n = (v..=v).find(|a| {
    if *a % (1024 * 1024) == 0 {
      println!("{a}");
    }
    regs[0] = *a;
    let mut vm = VM {
      prog: &prog,
      ip: 0,
      regs,
    };
    std::iter::from_fn(|| vm.step())
      .filter_map(|o| o)
      .eq(prog.iter().copied())
  }).unwrap();
  */
  
  //println!("{n}");
}