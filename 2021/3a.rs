use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let v = lines
    .take_while(|l| l.len() > 0)
    .map(|l| l.to_owned())
    .collect::<Vec<_>>();
    
  let w = v.iter().map(String::len).max().unwrap_or(0);
  let mut c = vec![0; w];
  for l in &v {
    for (i, d) in l.bytes().rev().enumerate() {
      c[i] += (d == '1' as u8) as usize;
    }
  }

  let t = (v.len() + 1) / 2;
  let gamma = c.iter().rev()
    .fold(0, |v, n| (v << 1) | ((*n >= t) as u32));
  let epsilon = ((1 << w) - 1) ^ gamma;
  
  println!("{}", gamma * epsilon);
}