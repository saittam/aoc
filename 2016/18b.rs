use std::io::BufRead;
use std::iter::once;
    
fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let row = once(false).chain(
    lines.next().expect("input").chars().map(|c| c == '^'))
    .chain(once(false))
    .collect::<Vec<bool>>();

  let (_, n) = (0..400000).fold((row, 0), |(r, n), _| {
    let n = n + r.iter().filter(|b| !**b).count() - 2;
    let r = once(false).chain(r.windows(3).map(|t| match t {
      [true, true, false] => true,
      [false, true, true] => true,
      [true, false, false] => true,
      [false, false, true] => true,
      _ => false,
    })).chain(once(false)).collect::<Vec<bool>>();
    (r, n)
  });
  
  println!("{}", n)
}