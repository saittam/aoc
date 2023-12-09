use std::io::BufRead;
use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq)]
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

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut packets = lines
    .filter(|l| l.len() > 0)
    .map(|l| Item::parse(&l).expect("parse"))
    .collect::<Vec<_>>();
  let d = [
    Item::List(vec![Item::List(vec![Item::Num(2)])]),
    Item::List(vec![Item::List(vec![Item::Num(6)])]),
  ];
  packets.extend(d.clone());
  packets.sort();
  let r = d.iter()
    .map(|d| packets.iter()
                    .position(|p| p == d)
                    .expect("position"))
    .map(|i| i + 1)
    .product::<usize>();
  println!("{}", r);
}