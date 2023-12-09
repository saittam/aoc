use std::io::BufRead;
use std::collections::VecDeque;

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

#[derive(Debug)]
struct Machine<'a> {
  prog: &'a [Insn],
  pc: usize,
  regs: [i64; 26],
  sink: i64,
  send: Option<i64>,
  ctr: usize,
  queue: VecDeque<i64>,
  status: Status,
}

impl<'a> Machine<'a> {
  fn new(prog: &'a [Insn]) -> Machine {
    Machine {
      prog,
      pc: 0,
      regs: [0; 26],
      sink: 0,
      send: None,
      ctr: 0,
      queue: VecDeque::new(),
      status: Status::Running(0),
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

  fn exec(&mut self, insn: Insn) -> Status {
    match insn {
      Insn::SND(a) => {
        self.ctr += 1;
        self.send = Some(self.val(a));
      }
      Insn::SET(d, s) => *self.dest(d) = self.val(s),
      Insn::ADD(d, s) => *self.dest(d) += self.val(s),
      Insn::MUL(d, s) => *self.dest(d) *= self.val(s),
      Insn::MOD(d, s) => *self.dest(d) %= self.val(s),
      Insn::RCV(a) =>
        if let Some(v) = self.queue.pop_front() {
          *self.dest(a) = v;
        } else {
          return Status::Waiting;
        }
      Insn::JGZ(a, o) => if self.val(a) > 0 {
        self.pc = self.target(o);
        return Status::Running(self.queue.len());
      }
    }
    self.pc += 1;
    Status::Running(self.queue.len())
  }

  fn step(&mut self) {
    self.status = if let Some(insn) = self.prog.get(self.pc) {
      self.exec(*insn)
    } else {
      Status::Halted
    };
  }

  fn recv(&mut self, v: i64) {
    self.queue.push_back(v);
    self.status = Status::Running(self.queue.len());
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Status {
  Halted,
  Waiting,
  Running(usize),
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let prog = lines
    .map(|l| l.parse::<Insn>().expect("insn"))
    .collect::<Vec<_>>();

  let mut machines = [
    Machine::new(&prog),
    Machine::new(&prog),
  ];
  machines[1].regs[Reg('p').idx()] = 1;
  loop {
    let i =
      (machines[1].status > machines[0].status) as usize;
    let m = &mut machines[i];
    if !matches!(m.status, Status::Running(_)) {
      break;
    }
    m.step();
    if let Some(v) = m.send.take() {
      machines[i ^ 1].recv(v);
    }
  }

  println!("{}", machines[1].ctr);
}