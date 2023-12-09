use std::io::BufRead;

fn tobits(s: &str) -> u128 {
  let mut b = s.split('-').map(|n| n.parse::<u8>().unwrap());
  let l = b.next().unwrap();
  let h = b.next().unwrap();
  (l..=h).fold(0, |a, n| a | (1 << n))
} 

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines()
    .map(|r| r.unwrap())
    .take_while(|l| l.len() > 0);

  let mut r = 0;
  for l in lines {
    let mut bits = l.split(',').map(tobits);
    let a = bits.next().unwrap();
    let b = bits.next().unwrap();
    let ab = a & b;
    r += (ab == a || ab == b) as usize;
  }

  println!("{}", r);
}