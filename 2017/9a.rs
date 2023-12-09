use std::io::BufRead;

fn score<I: Iterator<Item=char>>(mut i: I) -> Option<i32> {
  let mut n = 0;
  let mut tot = 0;
  loop {
    match i.next()? {
      '{' => {
        n += 1;
        tot += n;
      }
      '}' => n -= 1,
      '<' =>
        loop {
          match i.next()? {
            '>' => break,
            '!' => i.next(),
            _ => continue,
          };
        }
      ',' => (),
      _ => return None,
    }
    if n == 0 {
      return Some(tot);
    }
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let input = lines.next().expect("input");

  let n = score(input.chars()).expect("syntax");
  println!("{}", n);
}