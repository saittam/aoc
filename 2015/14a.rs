use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut v = Vec::new();
  for s in lines.take_while(|s| s.len() > 0) {
    let n = s.split(' ')
             .filter_map(|s| s.parse::<u32>().ok())
             .collect::<Vec<_>>();
    v.push((n[0], n[1], n[2]));
  }

  const T: u32 = 2503;
  let d = v.iter()
           .map(|(s, m, r)| s * m * (T / (m + r)) +
                            s * std::cmp::min(*m, T % (m + r)))
           .max().unwrap();
  
  println!("{}", d);
}