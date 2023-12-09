use std::io::BufRead;

#[derive(Clone)]
enum Num {
  Lit(i32),
  Pair(Box<Num>, Box<Num>),
}

impl std::fmt::Display for Num {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
    -> std::fmt::Result {
    match self {
      Num::Lit(v) => write!(f, "{}", v),
      Num::Pair(l, r) => write!(f, "[{},{}]", l, r),
    }
  }
}

impl Num {
  fn parse<I: Iterator<Item=char>>(s: &mut I)
    -> Option<Num> {
   Some(match s.next()? {
      '[' => {
         let l = Num::parse(s)?;
         let r = Num::parse(s)?;
         s.next();
         Num::Pair(Box::new(l), Box::new(r))
      }
      c if c.is_numeric() => {
        let v = s
          .take_while(|c| c.is_numeric())
          .fold(c.to_digit(10),
                |n, c| Some(n? * 10 + c.to_digit(10)?));
        Num::Lit(v? as i32)
      }
      _ => return None,
    })
  }
  
  fn split(&mut self) -> bool {
    let (res, num) = match self {
      Num::Lit(n) => {
        if *n >= 10 {
          (true,
           Some(Num::Pair(Box::new(Num::Lit(*n / 2)),
                          Box::new(Num::Lit(*n - *n / 2)))))
        } else {
          (false, None)
        }
      }
      Num::Pair(l, r) => (l.split() || r.split(), None),
    };
    
    if let Some(num) = num {
      *self = num;
    }
    
    res
  }

  fn prop_left(&mut self, n: i32) {
    match self {
      Num::Lit(ref mut v) => *v += n,
      Num::Pair(l, _) => l.prop_left(n),
    }
  }
  
  fn prop_right(&mut self, n: i32) {
    match self {
      Num::Lit(ref mut v) => *v += n,
      Num::Pair(_, r) => r.prop_right(n),
    }
  }

  fn explode(&mut self, lvl: usize)
    -> Option<(Option<i32>, Option<i32>)> {
    let (res, num) = match self {
      Num::Lit(_) => (None, None),
      Num::Pair(l, r) => {
        if lvl >= 4 {
          match (&**l, &**r) {
            (Num::Lit(ll), Num::Lit(lr)) =>
              (Some((Some(*ll), Some(*lr))),
               Some(Num::Lit(0))),
            _ => panic!("nesting"),
          }
        } else {
          (match l.explode(lvl + 1) {
            None =>
              match r.explode(lvl + 1) {
                None => None,
                Some((pl, pr)) => {
                  if let Some(p) = pl {
                    l.prop_right(p);
                  }
                  Some((None, pr))
                }
              }
            Some((pl, pr)) => {
              if let Some(p) = pr {
                r.prop_left(p);
              }
              Some((pl, None))
            }
          }, None)
        }
      }
    };
    
    if let Some(num) = num {
      *self = num;
    }
    
    res
  }

  fn add(a: Num, b: Num) -> Num {
    let mut n = Num::Pair(Box::new(a), Box::new(b));
    while n.explode(0).is_some() || n.split() {}
    n
  }
  
  fn magnitude(&self) -> i32 {
    match self {
      Num::Lit(v) => *v,
      Num::Pair(l, r) =>
        3 * l.magnitude() + 2 * r.magnitude(),
    }
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let n = lines
    .take_while(|l| l.len() > 0)
    .map(|s| Num::parse(&mut s.chars()).unwrap())
    .collect::<Vec<_>>();

  let mut m = 0;
  let mut i = n.iter();
  while let Some(a) = i.next() {
    for b in i.clone() {
      m = *[
        Num::add(a.clone(), b.clone()).magnitude(),
        Num::add(b.clone(), a.clone()).magnitude(),
        m
      ].iter().max().unwrap();
    }
  }
                   
  println!("{}", m);
}