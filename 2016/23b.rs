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
enum Arg {
  Reg(Reg),
  Imm(i32),
}

impl std::str::FromStr for Arg {
  type Err = ParseErr;
  fn from_str(s: &str) -> Result<Arg, ParseErr> {
    s.parse::<i32>().map(Arg::Imm)
    .or_else(|_| s.parse::<Reg>().map(Arg::Reg))
  }
}

#[derive(Debug, Clone, Copy)]
enum Insn {
  CPY(Arg, Arg),
  INC(Arg),
  DEC(Arg),
  JNZ(Arg, Arg),
  JMP(Arg),
  TGL(Arg),
}

impl Insn {
  fn toggle(&self) -> Insn {
    match *self {
      Insn::CPY(a1, a2) => Insn::JNZ(a1, a2),
      Insn::INC(a) => Insn::DEC(a),
      Insn::DEC(a) => Insn::INC(a),
      Insn::JNZ(a1, a2) => Insn::CPY(a1, a2),
      Insn::JMP(a) => Insn::INC(a),
      Insn::TGL(a) => Insn::INC(a),
    }
  }
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
      "CPY" => Ok(Insn::CPY(tok.arg()?, tok.arg()?)),
      "INC" => Ok(Insn::INC(tok.arg()?)),
      "DEC" => Ok(Insn::DEC(tok.arg()?)),
      "JNZ" => Ok(Insn::JNZ(tok.arg()?, tok.arg()?)),
      "JMP" => Ok(Insn::JMP(tok.arg()?)),
      "TGL" => Ok(Insn::TGL(tok.arg()?)),
      _ => Err(ParseErr),
    }
  }
}

struct State {
  pc: usize,
  regs: [i32; 4],
  prog: Vec<Insn>,
  sink: i32,
}

impl State {
  fn new(prog: Vec<Insn>) -> State {
    State { pc: 0, regs: [0; 4], prog, sink: 0 }
  }

  fn dest(&mut self, a: Arg) -> &mut i32 {
    match a {
      Arg::Reg(r) => &mut self.regs[r as usize],
      Arg::Imm(_) => &mut self.sink,
    }
  }

  fn val(&self, a: Arg) -> i32 {
    match a {
      Arg::Reg(r) => self.regs[r as usize],
      Arg::Imm(v) => v,
    }
  }

  fn target(&self, o: Arg) -> usize {
    (self.pc as i32 + self.val(o)) as usize
  }

  fn exec(&mut self, insn: Insn) {
    match insn {
      Insn::CPY(s, d) => *self.dest(d) = self.val(s),
      Insn::INC(r) => *self.dest(r) = self.val(r) + 1,
      Insn::DEC(r) => *self.dest(r) = self.val(r) - 1,
      Insn::JNZ(r, o) => if self.val(r) != 0 {
        self.pc = self.target(o);
        return;
      }
      Insn::JMP(o) => {
        self.pc = self.target(o);
        return;
      }
      Insn::TGL(o) => {
        let t = self.target(o);
        if let Some(i) = self.prog.get_mut(t) {
          *i = i.toggle();
        }
      }
    }
    self.pc += 1;
  }

  fn run(&mut self) {
    while let Some(insn) = self.prog.get(self.pc) {
      self.exec(*insn);
    }
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let prog = lines
    .map(|l| l.parse::<Insn>().expect("insn"))
    .collect::<Vec<_>>();

  let mut state = State::new(prog);
  *state.dest(Arg::Reg(Reg::A)) = 12;
  state.run();

  println!("{}", state.val(Arg::Reg(Reg::A)));
}