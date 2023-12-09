use std::io::BufRead;

enum PacketKind {
  Literal(u64),
  Sum(Vec<Packet>),
  Product(Vec<Packet>),
  Min(Vec<Packet>),
  Max(Vec<Packet>),
  Greater(Vec<Packet>),
  Less(Vec<Packet>),
  Equal(Vec<Packet>),
}

struct Packet {
  version: u8,
  kind: PacketKind,
}

struct BitIter<I: Iterator<Item=char>> {
  base: I,
  pending: u32,
  n: usize,
}

impl<I: Iterator<Item=char>> BitIter<I> {
  fn new(base: I) -> BitIter<I> {
    BitIter { base, pending: 0, n: 0 }
  }
}

impl<I: Iterator<Item=char>> Iterator for BitIter<I> {
  type Item = bool;
  
  fn next(&mut self) -> Option<bool> {
    if self.n == 0 {
      self.pending =
        self.base.next()?.to_digit(16).unwrap();
      self.n = 4;
    }
    
    self.n -= 1;
    Some((self.pending & (1 << self.n)) != 0)
  }
}

fn num(i: &mut dyn Iterator<Item=bool>, n: usize)
  -> Option<u64> {
  (0..n).fold(Some(0),
              |a, _| Some((a? << 1) | i.next()? as u64))
}

fn parse_sub(mut i: &mut dyn Iterator<Item=bool>)
  -> Option<Vec<Packet>> {
  Some(match i.next()? {
    false => {
      let n = num(&mut i, 15)? as usize;
      parse_vec(&mut i.take(n))
    }
    true => (0..num(&mut i, 11)?)
      .map(|_| parse(i))
      .collect::<Option<Vec<_>>>()?,
  })
}

fn parse(mut i: &mut dyn Iterator<Item=bool>)
  -> Option<Packet> {
  let version = num(i, 3)? as u8;
  let type_id = num(i, 3)? as u8;
  let kind = match type_id {
    0 => PacketKind::Sum(parse_sub(i)?),
    1 => PacketKind::Product(parse_sub(i)?),
    2 => PacketKind::Min(parse_sub(i)?),
    3 => PacketKind::Max(parse_sub(i)?),
    4 => {
      let mut v = 0;
      let mut more = true;
      while more {
        more = i.next()?;
        v = (v << 4) | num(i, 4)?;
      }
      PacketKind::Literal(v)
    }
    5 => PacketKind::Greater(parse_sub(i)?),
    6 => PacketKind::Less(parse_sub(i)?),
    7 => PacketKind::Equal(parse_sub(i)?),
    _ => unreachable!(),
  };
    
  Some(Packet { version, kind })
}

fn parse_vec(mut i: &mut dyn Iterator<Item=bool>) -> Vec<Packet> {
  let mut v = Vec::new();
  while let Some(p) = parse(i) {
    v.push(p);
  }
  v
}

fn eval(p: &Packet) -> u64 {
  match &p.kind {
    PacketKind::Sum(v) => v.iter().map(eval).sum(),
    PacketKind::Product(v) => v.iter().map(eval).product(),
    PacketKind::Min(v) => v.iter().map(eval).min().unwrap(),
    PacketKind::Max(v) => v.iter().map(eval).max().unwrap(),
    PacketKind::Literal(v) => *v,
    PacketKind::Greater(v) => (eval(&v[0]) > eval(&v[1])) as u64,
    PacketKind::Less(v) => (eval(&v[0]) < eval(&v[1])) as u64,
    PacketKind::Equal(v) => (eval(&v[0]) == eval(&v[1])) as u64,
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  let hex = lines.next().unwrap();
  let p = parse(&mut BitIter::new(hex.chars())).unwrap();
  let v = eval(&p);
  println!("{}", v);
}