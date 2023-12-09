use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  let input = lines.next().unwrap();
  
  let (a, n, s) = input.chars()
    .fold((0, 0, 1),
          |(a, n, s), c| {
            match c.to_digit(10) {
              Some(d) => (a, n * 10 + d as i64, s),
              None => (a + n * s, 0,
                       if c == '-' { -1 } else { 1 }),
              }
            }
          );

  println!("{}", a + n * s);
}