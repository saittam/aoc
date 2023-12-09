use std::io::BufRead;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

fn hash(n: u32) -> u64 {
  let mut hasher = DefaultHasher::new();
  hasher.write_u32(n);
  hasher.finish()
}

#[derive(Clone, PartialEq, Eq)]
struct State {
  decks: [VecDeque<u32>; 2],
  hash: u64,
}

type Cache = HashMap<u64, Vec<([VecDeque<u32>; 2], usize)>>;

impl State {
  fn new<'a, I>(i: &'a [I; 2]) -> State
  where
    &'a I: IntoIterator<Item=&'a u32>
  {
    let decks = [
      i[0].into_iter().cloned().collect::<VecDeque<u32>>(),
      i[1].into_iter().cloned().collect::<VecDeque<u32>>(),
    ];
    let hash = Self::hash_decks::<VecDeque<u32>>(&decks);
    State { decks, hash }
  }
  
  fn hash_decks<'a, I>(i: &'a [I; 2]) -> u64
  where
    &'a I: IntoIterator<Item=&'a u32>,
    <&'a I as IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
  {
    i[0].into_iter().rev().fold(0u64, |h, c| h.rotate_left(1) ^ hash(*c)) ^
    i[1].into_iter().rev().fold(0u64, |h, c| h.rotate_left(1) ^ !hash(*c))
  }

  fn round(&mut self, cache: &mut Cache) {
    let c = [self.decks[0].pop_front().unwrap(),
             self.decks[1].pop_front().unwrap()];
    self.hash = (self.hash ^ hash(c[0]) ^ !hash(c[1])).rotate_right(1);

    let winner = 
      if (0..2).all(|i| self.decks[i].len() >= c[i] as usize) {
        let mut rec = State::new::<VecDeque<u32>>(
          &[self.decks[0].iter().take(c[0] as usize).cloned().collect(),
            self.decks[1].iter().take(c[1] as usize).cloned().collect()]);
        rec.game(cache)
      } else {
        (c[0] < c[1]) as usize
      };
    
    let wdeck = &mut self.decks[winner];
    let h1 = hash(c[winner]).rotate_left(wdeck.len() as u32);
    wdeck.push_back(c[winner]);
    let h2 = hash(c[1 - winner]).rotate_left(wdeck.len() as u32);
    wdeck.push_back(c[1 - winner]);
    self.hash ^= h1 ^ h2;
  }

  fn game(&mut self, cache: &mut Cache) -> usize {
    let start = self.clone();
    if let Some(w) = self.lookup(cache) {
      return w;
    }

    let mut hashes = Vec::new();
    let winner = loop {
      hashes.push(self.hash);
      self.round(cache);

      if let Some(w) = self.decks.iter().position(|d| d.is_empty()) {
        self.cache(cache, 1 - w);
        break 1 - w;
      }

      if let Some(w) = self.lookup(cache) {
        return w;
      }

      if let Some(n) = hashes.iter().position(|h| *h == self.hash) {
        let mut prev = start.clone();
        for _ in 0..n {
          prev.round(cache);
        }

        if *self == prev {
          self.cache(cache, 0);
          break 0;
        } else {
          println!("hash collision {}", self.hash);
        }
      }
    };

    start.cache(cache, winner);
    winner
  }

  fn lookup(&self, cache: &mut Cache) -> Option<usize> {
    if let Some(l) = cache.get(&self.hash) {
      if let Some((_, w)) = l.iter().find(|(d, _)| *d == self.decks) {
        return Some(*w);
      }
    }
    None
  }

  fn cache(&self, cache: &mut Cache, winner: usize) {
    let e = cache.entry(self.hash).or_insert(Vec::new());
    if e.iter().find(|(d, _)| *d == self.decks).is_none() {
      e.push((self.decks.clone(), winner));
    }
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut decks = [ VecDeque::new(), VecDeque::new() ];
  for p in 0..2 {
    decks[p] = lines.by_ref().take_while(|l| l.len() > 0)
         .skip(1)
         .map(|l| l.parse::<u32>().unwrap())
         .collect::<VecDeque<_>>();
  }

  let mut state = State::new::<VecDeque<u32>>(&decks);
  let mut cache = HashMap::new();
  let winner = state.game(&mut cache);
  
  let s = state.decks[winner].iter().rev()
    .enumerate()
    .map(|(i, c)| (i + 1) * *c as usize)
    .sum::<usize>();
  println!("{}", s);
}
