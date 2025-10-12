use std::io::BufRead;
use std::collections::HashMap;
use std::collections::VecDeque;

trait Set<E> {
  type Iter<'a>: Iterator<Item=E> + 'a;
  fn empty() -> Self;
  fn add(&mut self, n: E);
  fn union(&mut self, other: &Self);
  fn intersect(&mut self, other: &Self);
  fn iter(&self) -> Self::Iter<'_>;
}

impl<'a, E> IntoIterator for &'a Set<E> {
  type Item = E;
  type IntoIter = Set::Iter<'a>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

struct Bits(u64);

struct BitsIter<'a> {
  bits: u64,
  _phantom: std::marker::PhantomData<'a>,
}

impl Set<u8> for Bits {
  type Iter<'a> = BitsIter<'a>;
  
  fn empty() -> Bits {
    Bits(0)
  }

  fn add(&mut self, n: u8) {
    *self.0 |= 1 << n;
  }

  fn union(&mut self, other: &Self) {
    *self.0 |= other;
  }

  fn intersect(&mut self, other: &Self) {
    *self.0 &= other;
  }

  fn iter<'a>(&'a self) -> Self::Iter<'a> {
    BitsIter(self)
  }
}

impl<'a> Iterator for BitsIter<'a> {
  type Item = u8;
  fn next(&mut self) -> Option<u8> {
    if self.bits == 0 {
      None
    } else {
      let n = self.bits.trailing_zeros() as u8;
      self.bits &= !(1 << n);
      Some(n)
    }
  }
}

enum Module {
  FlipFlop(bool),
  Conjunction(u64),
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut idmap = HashMap::new();
  let mut next_id = 0;
  let mut id = |s: &str| *idmap.entry(s.to_owned())
    .or_insert_with(|| {
      next_id += 1;
      assert!(next_id < 64);
      next_id
  });

  let mut broadcast = Vec::new();
  let mut modules = lines
    .filter_map(|l| {
      let mut wi = l.split_whitespace();
      let name = wi.next().expect("name");
      assert_eq!(wi.next(), Some("->"));
      let dest = wi
        .map(|w| id(w.trim_end_matches(',')))
        .collect::<Vec<_>>();
      let mut ni = name.chars();
      let kind = match ni.next() {
        Some('%') => Module::FlipFlop(false),
        Some('&') => Module::Conjunction(!0),
        _ if name == "broadcaster" => {
          broadcast = dest;
          return None;
        }
        t => panic!("bad module type {:?}", t),
      };
      Some((id(ni.as_str()), (kind, dest)))
    })
    .collect::<HashMap<_, _>>();

  let conn = modules.iter()
    .flat_map(|(n, (_, d))| d.iter().map(move |d| (*n, *d)))
    .chain(broadcast.iter().map(|d| (0, *d)))
    .collect::<Vec<_>>();
  for (n, d) in conn {
    let module = modules.get_mut(&d);
    if let Some((Module::Conjunction(i), _)) = module {
      *i &= !(1 << n);
    }
  }

  let logged = [ "ck", "dx", "jh", "cs" ]
    .iter()
    .map(|m| id(m))
    .collect::<Vec<_>>();
  let rx = id("rx");

'outer:
  for n in 1.. {
    let mut queue = broadcast.iter()
      .map(|n| (0, *n, false))
      .collect::<VecDeque<_>>();
    while let Some((s, d, p)) = queue.pop_front() {
      if d == rx && !p {
        println!("{}", n);
        break 'outer;
      }
      
      let (kind, dest) = if let Some(m) = modules.get_mut(&d) {
        m
      } else {
        continue;
      };
      
      let dp = match kind {
        Module::FlipFlop(e) => if p {
          continue;
        } else {
          *e = !*e;
          *e
        }
        Module::Conjunction(i) => {
          
          if logged.iter().find(|m| **m == d).is_some() && *i == !0 {
            println!("{}: {d} out {}", n, *i != !0);
          }
          
          *i = (*i & !(1 << s)) | ((p as u64) << s);
          *i != !0
        }
      };
      queue.extend(dest.iter().map(|n| (d, *n, dp)));
    }
  }

  // Idea: split into smaller problems:
  // - walk conjunction nodes backwards from rx to find n flipflops that influence output
  // - transitive closure of everything feeding into these flipfloos
  // - the latter should produce subproblems that can be solved independently
  // - cycle detection on each of the subproblems
}
