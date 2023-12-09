use std::io::BufRead;

struct Escape<I: Iterator<Item=char>> {
  i: I,
  e: Option<char>,
}

fn escape<I: Iterator<Item=char>>(i: I) -> Escape<I> {
  Escape { i, e: None }
}

impl<I: Iterator<Item=char>> Iterator for Escape<I> {
  type Item = char;
  
  fn next(&mut self) -> Option<char> {
    Some(match self.e {
      Some(c) => {
        self.e = None;
        c
      }
      None => {
        let c = self.i.next()?;
        match c {
          '"' | '\\' => {
            self.e = Some(c);
            '\\'
          }
          _ => c,
        }
      }
    })
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut raw_len = 0;
  let mut encoded_len = 0;
  for s in lines.take_while(|s| s.len() > 0) {
    raw_len += s.len();
    encoded_len += escape(s.chars()).count() + 2;
  }
      
  println!("{}", encoded_len - raw_len);
}