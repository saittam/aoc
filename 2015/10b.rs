use std::io::BufRead;
use std::iter::Peekable;

struct LookAndSay<I: Iterator<Item=char>> {
  i: Peekable<I>,
  c: std::vec::IntoIter<char>,
  n: Option<char>,
}

fn look_and_say<I: Iterator<Item=char>>(i: I) -> LookAndSay<I> {
  LookAndSay { i: i.peekable(), 
               c: Vec::new().into_iter(),
               n: None }
}

impl<I: Iterator<Item=char>> Iterator for LookAndSay<I> {
  type Item = char;
  
  fn next(&mut self) -> Option<char> {
    if let Some(c) = self.c.next() {
      return Some(c);
    }
    
    match self.n.take() {
      Some(c) => Some(c),
      None => {
        self.n = Some(self.i.next()?);
        let mut c = 1;
        while self.n == self.i.peek().cloned() {
          self.i.next();
          c += 1;
        }
        self.c = c.to_string().chars().collect::<Vec<_>>().into_iter();
        self.c.next()
      }
    }
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  let input = lines.next().unwrap();
  
  let mut seq = input;
  for _i in 0..50 {
    seq = look_and_say(seq.chars()).collect::<String>();
  }

  println!("{}", seq.len());
}