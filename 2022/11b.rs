use std::io::BufRead;

struct Monkey {
  items: Vec<usize>,
  operation: Box<dyn Fn(usize) -> usize>,
  test: Box<dyn Fn(usize) -> usize>,
  active: usize,
  div: usize,
}

fn argfn(s: &str) -> Box<dyn Fn(usize) -> usize> {
  if s == "old" {
    Box::new(|v| v)
  } else if let Ok(l) = s.parse::<usize>() {
    Box::new(move |_| l)
  } else  {
    panic!("arg {}", s)
  }
}

impl Monkey {
  fn parse<I: Iterator<Item=String>>(mut lines: I)
    -> Option<Monkey> {
    let items = lines.next()?
      .split_once(':')?.1
      .split(',')
      .map(|w| w.trim().parse::<usize>().ok())
      .collect::<Option<Vec<_>>>()?;
    let ol = lines.next()?;
    let mut opw = ol
      .split_once('=')?.1
      .trim()
      .split(' ');
    let a = argfn(opw.next()?);
    let o = opw.next()?;
    let b = argfn(opw.next()?);
    let operation: Box<dyn Fn(usize) -> usize> = match o {
      "+" => Box::new(move |o| a(o) + b(o)),
      "*" => Box::new(move |o| a(o) * b(o)),
      _ => return None,
    };
    let mut tn = lines.map(
      |l| l.split(' ').last()?.parse::<usize>().ok())
      .map_while(|n| n);
    let (div, pos, neg) = (tn.next()?, tn.next()?, tn.next()?);
    let test = 
      Box::new(move |v| if v % div == 0 { pos } else { neg });
    let active = 0;
    Some(Monkey { items, operation, test, active, div })
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut monkeys = Vec::new();
  while let Some(l) = lines.next() {
    if l.len() == 0 {
      break;
    }
    assert!(l.starts_with("Monkey"));
    let m = Monkey::parse(
      lines.by_ref().take_while(|l| l.len() > 0))
      .expect("monkey");
    lines.next();
    monkeys.push(m);
  }

  let modulus =
    monkeys.iter().map(|m| m.div).product::<usize>(); 

  for _ in 0..10000 {
    for i in 0..monkeys.len() {
      let mut items = Vec::new();
      std::mem::swap(&mut items, &mut monkeys[i].items);
      for n in items {
        monkeys[i].active += 1;
        let nn = (monkeys[i].operation)(n) % modulus;
        let dest = (monkeys[i].test)(nn);
        monkeys[dest].items.push(nn);
      }
    }
  }

  let mut a = 
    monkeys.iter().map(|m| m.active).collect::<Vec<_>>();
  a.sort();
  let r = a.iter().rev().take(2).product::<usize>();
  println!("{}", r);
}