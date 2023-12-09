use std::io::BufRead;
use std::iter::Peekable;

fn term<I: Iterator<Item=char>>(s: &mut Peekable<I>)
    -> Option<u64> {
  match s.next()? {
    '(' => expr(s),
    c if c.is_digit(10) => {
      let mut val = c.to_digit(10)? as u64;
      while let Some(d) = s.peek().and_then(|c| c.to_digit(10)) {
        val = val * 10 + d as u64;
        s.next();
      }
      Some(val)
    },
    _ => None,
  }
}

fn expr<I: Iterator<Item=char>>(s: &mut Peekable<I>)
    -> Option<u64> {
  let mut r = term(s)?;
  while let Some(c) = s.next() {
    match c {
      ')' => break,
      '+' => r += term(s)?,
      '*' => r *= term(s)?,
      _ => return None,
    }
  }
  Some(r)
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines();
  
  let s = lines.map(|r| r.unwrap())
    .take_while(|l| l.len() > 0)
    .map(|l| expr(&mut l.chars()
                  .filter(|c| !c.is_whitespace())
                  .peekable()).unwrap())
    .sum::<u64>();
                  
  println!("{}", s);
}