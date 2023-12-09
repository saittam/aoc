use std::io::BufRead;

enum PacketKind {
  Literal(u64),
  Operator(u8, Vec<Packet>),
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

fn num<I: Iterator<Item=bool>>(i: &mut I, n: usize)
  -> Option<u64> {
  (0..n).fold(Some(0),
              |a, _| Some((a? << 1) | i.next()? as u64))
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

fn parse(mut i: &mut dyn Iterator<Item=bool>)
  -> Option<Packet> {
  let version = num(&mut i, 3)? as u8;
  let type_id = num(&mut i, 3)? as u8;
  let kind = match type_id {
    4 => {
      let mut v = 0;
      let mut more = true;
      while more {
        more = i.next()?;
        v = (v << 4) | num(&mut i, 4)?;
      }
      PacketKind::Literal(v)
    }
    _ => {
      let v = match i.next()? {
        false => {
          let n = num(&mut i, 15)? as usize;
          parse_vec(&mut i.take(n))
        }
        true => (0..num(&mut i, 11)?)
          .map(|_| parse(i))
          .collect::<Option<Vec<_>>>()?,
      };
      PacketKind::Operator(type_id, v)
    }
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

fn versionsum(p: &Packet) -> usize {
  p.version as usize + match p.kind {
    PacketKind::Literal(_) => 0,
    PacketKind::Operator(_, ref s) =>
      s.iter().map(versionsum).sum(),
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  let hex = lines.next().unwrap();
  let p = parse_vec(&mut BitIter::new(hex.chars()));
  let s = p.iter().map(versionsum).sum::<usize>();
  println!("{}", s);
}