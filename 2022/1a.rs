use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut r = Vec::new();
  loop {
    let c = lines.by_ref()
      .take_while(|l| l.len() > 0)
      .map(|l| l.parse::<usize>().expect("bad input"))
      .collect::<Vec<_>>();
    if c.is_empty() {
      break;
    }
    r.push(c);
  }

  let mc = r.iter()
    .map(|c| c.iter().sum::<usize>())
    .max()
    .expect("no input");
  println!("{}", mc);
}