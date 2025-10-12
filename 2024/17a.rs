use std::io::BufRead;

type Val = u32;

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
  let regs = regs.try_into().expect("regs");

  let prog = lines.next().expect("prog")
    .split(|c: char| !c.is_digit(10))
    .filter(|w| !w.is_empty())
    .map(|w| w.parse::<Val>().expect("num"))
    .collect::<Vec<_>>();

  let mut vm = VM {
    prog: &prog,
    ip: 0,
    regs,
  };
  let output = std::iter::from_fn(|| vm.step())
    .filter_map(|o| o)
    .map(|o| format!("{o}"))
    .collect::<Vec<String>>()
    .join(",");
  
  println!("{output}");
}