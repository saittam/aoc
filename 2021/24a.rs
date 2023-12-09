use std::io::BufRead;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::collections::VecDeque;

type Reg = usize;

fn parse_reg(s: &str) -> Option<usize> {
  match s {
    "w" => Some(0),
    "x" => Some(1),
    "y" => Some(2),
    "z" => Some(3),
    _ => None,
  }
}

#[derive(Clone, Copy, Debug)]
enum Arg {
  Reg(Reg),
  Imm(i64),
}

impl Arg {
  fn parse(s: &str) -> Option<Arg> {
    if let Some(r) = parse_reg(s) {
      Some(Arg::Reg(r))
    } else {
      Some(Arg::Imm(s.parse::<i64>().ok()?))
    }
  }    
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RegUse {
  Read,
  Clobber,
  Ignore,
}

impl RegUse {
  fn chain(self, u: RegUse) -> RegUse {
    match (self, u) {
      (RegUse::Ignore, u) => u,
      (u, _) => u,
    }
  }
}

#[derive(Debug)]
enum Insn {
  Inp(Reg),
  Add(Reg, Arg),
  Mul(Reg, Arg),
  Div(Reg, Arg),
  Mod(Reg, Arg),
  Eql(Reg, Arg),
}

impl Insn {
  fn parse(s: &str) -> Option<Insn> {
    let mut i = s.split_whitespace();
    match i.next()? {
      "inp" => Some(Insn::Inp(parse_reg(i.next()?)?)),
      "add" => Some(Insn::Add(parse_reg(i.next()?)?,
                              Arg::parse(i.next()?)?)),
      "mul" => Some(Insn::Mul(parse_reg(i.next()?)?,
                              Arg::parse(i.next()?)?)),
      "div" => Some(Insn::Div(parse_reg(i.next()?)?,
                              Arg::parse(i.next()?)?)),
      "mod" => Some(Insn::Mod(parse_reg(i.next()?)?,
                              Arg::parse(i.next()?)?)),
      "eql" => Some(Insn::Eql(parse_reg(i.next()?)?,
                              Arg::parse(i.next()?)?)),
      _ => None
    }
  }
  
  fn reguse(&self) -> [RegUse; 4] {
    let u = [RegUse::Ignore; 4];
    let ru = |mut u: [RegUse; 4], r: Reg, ru| { 
      u[r] = ru;
      u
    };
    let clobber = |u, r| ru(u, r, RegUse::Clobber);
    let read = |u, r| ru(u, r, RegUse::Read);
    let readarg = |u: [RegUse; 4], a: Arg| match a {
      Arg::Reg(r) => read(u, r),
      Arg::Imm(_) => u,
    };
    match *self {
      Insn::Inp(r) => clobber(u, r),
      Insn::Add(r, a) => read(readarg(u, a), r),
      Insn::Mul(r, Arg::Imm(0)) => clobber(u, r),
      Insn::Mul(r, a) => read(readarg(u, a), r),
      Insn::Div(r, a) => read(readarg(u, a), r),
      Insn::Mod(r, a) => read(readarg(u, a), r),
      Insn::Eql(r, Arg::Reg(a)) if r == a => clobber(u, r),
      Insn::Eql(r, a) => read(readarg(u, a), r),
    }
  }
}

struct VM {
  regs: [i64; 4],
  input: VecDeque<i64>,
}

impl VM {
  fn reg(&mut self, r: Reg) -> &mut i64 {
    &mut self.regs[r]
  }
  
  fn val(&self, a: Arg) -> i64 {
    match a {
      Arg::Reg(r) => self.regs[r],
      Arg::Imm(v) => v,
    }
  }
  
  fn run(&mut self, prog: &[Insn]) {
    for insn in prog {
      match *insn {
        Insn::Inp(a) =>
          *self.reg(a) = self.input.pop_front().unwrap(),
        Insn::Add(a, b) => *self.reg(a) += self.val(b),
        Insn::Mul(a, b) => *self.reg(a) *= self.val(b),
        Insn::Div(a, b) => *self.reg(a) /= self.val(b),
        Insn::Mod(a, b) => *self.reg(a) %= self.val(b),
        Insn::Eql(a, b) => *self.reg(a) =
          ({*self.reg(a)} == self.val(b)) as i64,
      }
    }
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let prog = lines
    .take_while(|l| l.len() > 0)
    .map(|s| Insn::parse(&s).unwrap())
    .collect::<Vec<_>>();
  let mut pi = prog.iter().enumerate()
    .filter_map(
      |(k, i)| if let Insn::Inp(_) = i { Some(k) } 
               else { None })
    .collect::<Vec<_>>();
  pi.push(prog.len());
  let proglets = pi.windows(2)
    .map(|w| &prog[w[0]..w[1]])
    .collect::<Vec<_>>();
  assert_eq!(proglets.len(), 14);
  assert_eq!(pi[0], 0);
  
  let mut rmask = Vec::new();
  for p in &proglets {
    let rm = p.iter()
      .map(Insn::reguse)
      .fold([RegUse::Ignore; 4],
            |u, iu| [u[0].chain(iu[0]),
                     u[1].chain(iu[1]),
                     u[2].chain(iu[2]),
                     u[3].chain(iu[3])]);
    rmask.push(rm);
  }
  let mut rml = [RegUse::Clobber; 4];
  rml[3] = RegUse::Read;
  rmask.push(rml);
 
  let mut pfxm = BTreeMap::new();
  pfxm.insert([0; 4], 0u64);
  for (k, (p, rm)) in proglets.iter()
                              .zip(rmask.iter().skip(1))
                              .enumerate() {
    println!("k {} pfxm len {} 9^k {}", k, pfxm.len(), 9u64.pow(k as u32));
    let mut pfxmn = BTreeMap::new();
    println!("{:x?}", pfxm.iter().take(1).collect::<Vec<_>>());
    for (r, pfx) in pfxm.iter() {
      for i in (1..=9) {
        let mut vm = VM {
          regs: r.clone(), 
          input: [i as i64].iter().cloned().collect(),
        };
        vm.run(&p);
        
        for (v, m) in vm.regs.iter_mut().zip(rm.iter()) {
          *v = if *m == RegUse::Clobber { 0 } else { *v };
        }
        
        match pfxmn.entry(vm.regs) {
          Entry::Vacant(e) => {
            e.insert((pfx << 4) | i as u64);
          }
          Entry::Occupied(mut e) => {
            let pfxn = u64::max(*e.get(), (pfx << 4) | i as u64);
            e.insert(pfxn);
          }
        }
      }
    }
    pfxm = pfxmn;
  }
  
  println!("{:x?}", pfxm.iter().take(5).collect::<Vec<_>>());

  let (_, sn) = pfxm.iter()
    .filter(|(r, _)| r[3] == 0)
    .max_by_key(|(_, sn)| *sn)
    .expect("no z == 0 result");
  
  println!("{:x}", sn);
}::<Vec<_>>());
  
  let ci = [6, 1, 9, 1, 9, 9, 9, 7, 2, 9, 9, 4, 3, 2];
  let mut vm = VM {
    regs: [0; 4],
    input: ci.iter().cloned().collect(),
  };
  vm.run(&prog);
  println!("{:?}", vm.regs);

  let (_, sn) = pfxm.iter()
    .filter(|(r, _)| r[3] == 0)
    .max_by_key(|(_, sn)| *sn)
    .expect("no z == 0 result");
  
  println!("{:x}", sn);
}

/*
  
inp w
mul x 0
add x z
mod x 26
div z 26
add x -11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 5
mul y x
add z y


z /= k1{26,1}
x = ((z % 26) + k2{-11}) != input
z *= 25 * x + 1
z += (input + k3{5}) * x

*/z *= 25 * x + 1
z += (input + k3{5}) * x

*/ == 0)
    .max_by_key(|(_, sn)| sn.clone())
    .expect("no z == 0 result");
  
  let s = sn.iter()
            .map(|d| std::char::from_digit(*d as u32, 10).unwrap())
            .collect::<String>();
  println!("{}", s);
}

/*
  
inp w
mul x 0
add x z
mod x 26
div z 26
add x -11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 5
mul y x
add z y


z /= k1{26,1}
x = ((z % 26) + k2{-11}) != input
z *= 25 * x + 1
z += (input + k3{5}) * x

*/