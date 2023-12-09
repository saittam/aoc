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
      c @ 'a'..='z' => Ok(Reg(c)),
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
  SND(Arg),
  SET(Arg, Arg),
  ADD(Arg, Arg),
  MUL(Arg, Arg),
  MOD(Arg, Arg),
  RCV(Arg),
  JGZ(Arg, Arg),
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
      "SND" => Ok(Insn::SND(tok.arg()?)),
      "SET" => Ok(Insn::SET(tok.arg()?, tok.arg()?)),
      "ADD" => Ok(Insn::ADD(tok.arg()?, tok.arg()?)),
      "MUL" => Ok(Insn::MUL(tok.arg()?, tok.arg()?)),
      "MOD" => Ok(Insn::MOD(tok.arg()?, tok.arg()?)),
      "RCV" => Ok(Insn::RCV(tok.arg()?)),
      "JGZ" => Ok(Insn::JGZ(tok.arg()?, tok.arg()?)),
      _ => Err(ParseErr),
    }
  }
}

struct State {
  pc: usize,
  regs: [i64; 26],
  freq: i64,
  sink: i64,
  rcvd: Option<i64>,
}

impl State {
  fn new() -> State {
    State { pc: 0, regs: [0; 26], freq: 0, sink: 0, rcvd: None }
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
      Insn::SND(a) => self.freq = self.val(a),
      Insn::SET(d, s) => *self.dest(d) = self.val(s),
      Insn::ADD(d, s) => *self.dest(d) += self.val(s),
      Insn::MUL(d, s) => *self.dest(d) *= self.val(s),
      Insn::MOD(d, s) => *self.dest(d) %= self.val(s),
      Insn::RCV(a) =>
        if self.val(a) != 0 {
          self.rcvd = Some(self.freq);
        }
      Insn::JGZ(a, o) => if self.val(a) > 0 {
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
    if let Some(f) = state.rcvd {
      println!("{}", f);
      break;
    }
  }
}