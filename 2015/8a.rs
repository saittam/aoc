use std::io::BufRead;

struct Unescape<I: Iterator<Item=char>> {
  i: I,
}

fn unescape<I: Iterator<Item=char>>(i: I) -> Unescape<I> {
  Unescape { i }
}

impl<I: Iterator<Item=char>> Iterator for Unescape<I> {
  type Item = char;
  
  fn next(&mut self) -> Option<char> {
    loop {
      break Some(match self.i.next()? {
        '"' => continue,
        '\\' => match self.i.next()? {
          'x' => {
            let code =
              (self.i.next()?.to_digit(16).unwrap() << 4) |
               self.i.next()?.to_digit(16).unwrap();
            std::char::from_u32(code).unwrap()
          }
          c => c,
        }
        c => c,
      })
    }
  }
}
  
fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut raw_len = 0;
  let mut parsed_len = 0;
  for s in lines.take_while(|s| s.len() > 0) {
    raw_len += s.len();
    parsed_len += unescape(s.chars()).count();
  }
      
  println!("{}", raw_len - parsed_len);
}