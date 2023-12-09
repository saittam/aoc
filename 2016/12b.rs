use std::io::BufRead;

#[derive(Debug)]
struct ParseErr;

#[derive(Debug, Clone, Copy)]
enum Reg {
  A = 0,
  B = 1,
  C = 2,
  D = 3,
}

impl std::str::FromStr for Reg {
  type Err = ParseErr;
  fn from_str(s: &str) -> Result<Reg, ParseErr> {
    match s {
      "a"|"A" => Ok(Reg::A),
      "b"|"B" => Ok(Reg::B),
      "c"|"C" => Ok(Reg::C),
      "d"|"D" => Ok(Reg::D),
      _ => Err(ParseErr),
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum Insn {
  CPYR(Reg, Reg),
  CPYI(u32, Reg),
  INC(Reg),
  DEC(Reg),
  JNZ(Reg, isize),
  JMP(isize),
}

struct Tokenizer<'a, I: Iterator<Item=&'a str>>(I);

fn tokenizer<'b>(s: &'b str)
  -> Tokenizer<'b, impl Iterator<Item=&'b str>> {
    Tokenizer(s.split_whitespace()
               .map(|w| w.trim_end_matches(',')))
}

impl<'a, I: Iterator<Item=&'a str>> Tokenizer<'a, I> {
  fn next(&mut self) -> Result<&'a str, ParseErr> {
    self.0.next().ok_or(ParseErr)
  }

  fn reg(&mut self) -> Result<Reg, ParseErr> {
    self.next()?.parse()
  }

  fn offset(&mut self) -> Result<isize, ParseErr> {
    self.next()?.parse::<isize>().map_err(|_| ParseErr)
  }
}

impl std::str::FromStr for Insn {
  type Err = ParseErr;
  fn from_str(s: &str) -> Result<Insn, ParseErr> {
    let mut tok = tokenizer(s);
    match tok.next()?.to_uppercase().as_str() {
      "CPY" => {
        let a = tok.next()?;
        let d = tok.reg()?;
        if let Ok(i) = a.parse::<u32>() {
          Ok(Insn::CPYI(i, d))
        } else {
          Ok(Insn::CPYR(a.parse::<Reg>()?, d))
        }
      }
      "INC" => Ok(Insn::INC(tok.reg()?)),
      "DEC" => Ok(Insn::DEC(tok.reg()?)),
      "JNZ" => {
        let a = tok.next()?;
        let offset = tok.offset()?;
        if let Ok(i) = a.parse::<u32>() {
          Ok(Insn::JMP(if i != 0 { offset } else { 0 }))
        } else {
          Ok(Insn::JNZ(a.parse::<Reg>()?, offset))
        }
      }
      _ => Err(ParseErr),
    }
  }
}

impl std::ops::Index<Reg> for [u32; 4] {
  type Output = u32;
  fn index(&self, r: Reg) -> &u32 {
    &self[r as usize]
  }
}

impl std::ops::IndexMut<Reg> for [u32; 4] {
  fn index_mut(&mut self, r: Reg) -> &mut u32 {
    &mut self[r as usize]
  }
}

struct State {
  pc: usize,
  regs: [u32; 4],
}

impl State {
  fn new() -> State {
    State { pc: 0, regs: [0; 4] }
  }

  fn exec(&mut self, insn: Insn) {
    match insn {
      Insn::CPYR(s, d) => self.regs[d] = self.regs[s],
      Insn::CPYI(i, d) => self.regs[d] = i,
      Insn::INC(reg) => self.regs[reg] += 1,
      Insn::DEC(reg) => self.regs[reg] -= 1,
      Insn::JNZ(reg, offset) => if self.regs[reg] != 0 {
        self.pc = (self.pc as isize + offset) as usize;
        return;
      }
      Insn::JMP(offset) => {
        self.pc = (self.pc as isize + offset) as usize;
        return;
      }
    }
    self.pc += 1;
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let prog = lines
    .map(|l| l.parse::<Insn>().expect("insn"))
    .collect::<Vec<_>>();

  let mut state = State::new();
  state.regs[Reg::C] = 1;
  while let Some(insn) = prog.get(state.pc) {
    state.exec(*insn);
  }

  println!("{}", state.regs[Reg::A]);
}