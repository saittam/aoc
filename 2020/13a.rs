use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut tb = String::new();
  handle.read_line(&mut tb);
  let t = tb.trim().parse::<u64>().unwrap();
  
  let mut bb = String::new();
  handle.read_line(&mut bb);
  let buses = bb.trim().split(',')
    .filter(|s| *s != "x")
    .map(|s| s.parse::<u64>().unwrap())
    .collect::<Vec<u64>>();
  
  let (w, b) = buses.iter()
    .map(|b| (b - (t % b), b))
    .min().unwrap();

  println!("{}", w * b);
}