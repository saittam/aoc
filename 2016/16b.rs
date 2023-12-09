use std::io::BufRead;

enum State<'a> {
  Fwd(std::slice::Iter<'a, bool>),
  Inv(std::iter::Rev<std::slice::Iter<'a, bool>>),
}

impl<'a> State<'a> {
  fn next(&mut self) -> Option<bool> {
    match self {
      State::Fwd(iter) => iter.next().cloned(),
      State::Inv(iter) => iter.next().map(|b| !b),
    }
  }
}

struct DiskIter<'a> {
  seed: &'a[bool],
  state: State<'a>,
  count: usize,
}

impl<'a> DiskIter<'a> {
  fn new(seed: &'a [bool]) -> DiskIter<'a> {
    let state = State::Fwd(seed.iter());
    let count = 0;
    DiskIter { seed, state, count }
  }
}

// (((S 0 s') 0 (S 1 s')) 0 ((S 0 s') 1 (S 1 s'))) 0 ...
// 0 0 1 0 0 1 1 0 0 0 1 1 0 1 1  0  0 0 1 0 0 1 1 1 0 0 1 1 0 1 
//        0
//    0       1
//  0   1   0   1
// 0 1 0 1 0 1 0 1
//
// cto(n): 0, 1, 0, 2, 0, 1, 0, 3, ...
impl<'a> Iterator for DiskIter<'a> {
  type Item = bool;
  fn next(&mut self) -> Option<bool> {
    self.state.next().or_else(|| {
      self.state = match &self.state {
        State::Fwd(_) => State::Inv(self.seed.iter().rev()),
        State::Inv(_) => State::Fwd(self.seed.iter()),
      };
      let val = self.count;
      self.count += 1;
      Some((val & (2 << val.trailing_ones())) != 0)
    })
  }
}

struct ParPairIter<I: Iterator<Item=bool>>(I);
  
impl<I: Iterator<Item=bool>> Iterator for ParPairIter<I> {
  type Item = bool;
  fn next(&mut self) -> Option<bool> {
    Some(self.0.next()? == self.0.next()?)
  }
}
    
fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let seed = lines.next().expect("seed").chars()
    .map(|c| c.to_digit(2).expect("bit") != 0)
    .collect::<Vec<_>>();

  const LEN: usize = 35651584;
  
  let mut len = LEN;
  let mut iter: Box<dyn Iterator<Item=bool>> = 
    Box::new(DiskIter::new(&seed));
  while len % 2 == 0 {
    iter = Box::new(ParPairIter(iter));
    len /= 2;
  }

  let result = iter
    .map(|b| char::from_digit(b as u32, 2).unwrap())
    .take(len).collect::<String>();
  
  println!("{}", result)
}