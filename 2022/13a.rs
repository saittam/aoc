use std::io::BufRead;
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
enum Item {
  Num(u32),
  List(Vec<Item>),
}

impl Item {
  fn parse(s: &str) -> Option<Item> {
    Item::parsei(&mut s.chars().peekable())
  }

  fn parsei<I>(i: &mut std::iter::Peekable<I>) -> Option<Item>
  where
    I: Iterator<Item=char>
  {
    Some(match i.next()? {
      '[' => {
        let mut v = Vec::new();
        while let Some(c) = i.peek() {
          match c {
            ']' => {
              i.next();
              break;
            }
            _ => {
              v.push(Item::parsei(i)?);
              match i.next()? {
                ',' => continue,
                ']' => break,
                _ => return None,
              }
            }
          }
        }
        Item::List(v)
      }
      c => {
        let mut n = c.to_digit(10)?;
        while let Some(c) = i.peek() {
          if let Some(d) = c.to_digit(10) {
            n = n * 10 + d;
            i.next();
          } else {
            break;
          }
        }
        Item::Num(n)
      }
    })
  }
}

impl Ord for Item {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
     match (self, other) {
       (Item::Num(l), Item::Num(r)) => l.cmp(r),
       (Item::List(l), Item::List(r)) => l.cmp(r),
       (Item::List(l), Item::Num(r)) =>
         l.cmp(&vec![Item::Num(*r)]),
       (Item::Num(l), Item::List(r)) =>
         vec![Item::Num(*l)].cmp(r),
     }
  }
}

impl PartialOrd for Item {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn next<I: Iterator<Item=String>>(lines: &mut I)
    -> Option<(String, String)> {
  let (l, r) = (lines.next()?, lines.next()?);
  lines.next();
  if l.len() > 0 || r.len() > 0 {
    Some((l, r))
  } else {
    None
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut sum = 0;
  for i in 1.. {
    if let Some((l, r)) = next(&mut lines) {
      let li = Item::parse(&l).expect("parse");
      let ri = Item::parse(&r).expect("parse");
      if li < ri {
        sum += i;
      }
    } else {
      break;
    }
  }
  println!("{}", sum);
}