use std::io::BufRead;
use std::str::FromStr;
use std::iter::Iterator;

#[derive(Clone)]
enum Dir {
  UP,
  DOWN,
  LEFT,
  RIGHT,
}

#[derive(Clone)]
struct Segment {
  dir: Dir,
  len: usize,
}

impl FromStr for Segment {
  type Err = &'static str;
  
  fn from_str(s: &str) -> Result<Segment, Self::Err> {
    let dir = match s.chars().nth(0).ok_or("missing dir")? {
      'U' => Dir::UP,
      'D' => Dir::DOWN,
      'L' => Dir::LEFT,
      'R' => Dir::RIGHT,
      _ => return Err("invalid dir")
    };
    
    Ok(Segment {
      dir: dir,
      len: s[1..].parse::<usize>().or(Err("invalid len"))?
    })
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
  x: isize,
  y: isize,
}

fn pos(x: isize, y: isize) -> Pos {
  Pos { x: x, y: y }
}

struct PosIterator<'a, I> where
I: Iterator<Item = &'a Segment> {
  segment_iter: I,
  pos: Pos,
  seg: Option<Segment>,
}

fn pos_iterator<'a, I>(segment_iter: I) -> PosIterator<'a, I> where
I: Iterator<Item = &'a Segment> {
  PosIterator {
    segment_iter: segment_iter,
    pos: pos(0, 0),
    seg: None,
  }
}

impl<'a, I> Iterator for PosIterator<'a, I> where
I: Iterator<Item = &'a Segment> {
  type Item = Pos;
  
  fn next(&mut self) -> Option<Pos> {
    loop {
      if let Some(ref mut s) = self.seg {
        if s.len > 0 {
          s.len -= 1;
          match s.dir {
            Dir::UP => self.pos.y += 1,
            Dir::DOWN => self.pos.y -= 1,
            Dir::LEFT => self.pos.x -= 1,
            Dir::RIGHT => self.pos.x += 1,
          }
          return Some(self.pos.clone());
        }
      }
      
      self.seg = match self.segment_iter.next() {
        Some(s) => Some(s.clone()),
        None => return None,
      };
    }
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
    
  let mut buf = String::new(); 
  handle.read_line(&mut buf);
  let first = buf.split(',')
    .map(|s| s.trim().parse::<Segment>().unwrap())
    .collect::<Vec<Segment>>();

  buf = String::new();
  handle.read_line(&mut buf);
  let second = buf.split(',')
    .map(|s| s.trim().parse::<Segment>().unwrap())
    .collect::<Vec<Segment>>();

  let mut r = std::collections::HashSet::<Pos>::new();
  for p in pos_iterator(first.iter()) {
    r.insert(p);
  }
  
  let m = pos_iterator(second.iter())
    .filter(|p| r.contains(p))
    .map(|p| p.x.abs() + p.y.abs())
    .min().unwrap();
  
  println!("{}", m);
}