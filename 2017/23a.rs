use std::io::BufRead;

#[derive(Debug)]
struct ParseErr;

#[derive(Debug, Clone, Copy)]
struct Reg(char);

impl Reg {
  fn idx(&self) -> usize {
    (self.0 as u32 - 'a' as u32) as usize
  }
}

impl std::str::FromStr for Reg {
  type Err = ParseErr;
  fn from_str(s: &str) -> Result<Reg, ParseErr> {
    match s.chars().next().ok_or(ParseErr)? {
      c @ 'a'..='h' => Ok(Reg(c)),
      _ => Err(ParseErr),
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum Arg {
  Reg(Reg),
  Imm(i64),
}

impl std::str::FromStr for Arg {
  type Err = ParseErr;
  fn from_str(s: &str) -> Result<Arg, ParseErr> {
    s.parse::<i64>().map(Arg::Imm)
    .or_else(|_| s.parse::<Reg>().map(Arg::Reg))
  }
}

#[derive(Debug, Clone, Copy)]
enum Insn {
  SET(Arg, Arg),
  SUB(Arg, Arg),
  MUL(Arg, Arg),
  JNZ(Arg, Arg),
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

  fn arg(&mut self) -> Result<Arg, ParseErr> {
    self.next()?.parse()
  }
}

impl std::str::FromStr for Insn {
  type Err = ParseErr;
  fn from_str(s: &str) -> Result<Insn, ParseErr> {
    let mut tok = tokenizer(s);
    match tok.next()?.to_uppercase().as_str() {
      "SET" => Ok(Insn::SET(tok.arg()?, tok.arg()?)),
      "SUB" => Ok(Insn::SUB(tok.arg()?, tok.arg()?)),
      "MUL" => Ok(Insn::MUL(tok.arg()?, tok.arg()?)),
      "JNZ" => Ok(Insn::JNZ(tok.arg()?, tok.arg()?)),
      _ => Err(ParseErr),
    }
  }
}

struct State {
  pc: usize,
  regs: [i64; 26],
  sink: i64,
  mul_cnt: usize,
}

impl State {
  fn new() -> State {
    State {
      pc: 0,
      regs: [0; 26],
      sink: 0,
      mul_cnt: 0,
    }
  }

  fn dest(&mut self, a: Arg) -> &mut i64 {
    match a {
      Arg::Reg(r) => &mut self.regs[r.idx()],
      Arg::Imm(_) => &mut self.sink,
    }
  }

  fn val(&self, a: Arg) -> i64 {
    match a {
      Arg::Reg(r) => self.regs[r.idx()],
      Arg::Imm(v) => v,
    }
  }

  fn target(&self, o: Arg) -> usize {
    (self.pc as i64 + self.val(o)) as usize
  }

  fn exec(&mut self, insn: Insn) {
    match insn {
      Insn::SET(d, s) => *self.dest(d) = self.val(s),
      Insn::SUB(d, s) => *self.dest(d) -= self.val(s),
      Insn::MUL(d, s) => {
        self.mul_cnt += 1;
        *self.dest(d) *= self.val(s);
      }
      Insn::JNZ(a, o) => if self.val(a) != 0 {
        self.pc = self.target(o);
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
  while let Some(insn) = prog.get(state.pc) {
    state.exec(*insn);
  }

  println!("{}", state.mul_cnt);
}