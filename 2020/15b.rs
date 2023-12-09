use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut buf = String::new();
  handle.read_line(&mut buf);
  let mut v = buf.trim().split(',')
    .map(|s| s.parse::<u32>().unwrap())
    .collect::<Vec<u32>>();
    
  let mut m = Vec::new();
  for (i, e) in v.iter().enumerate() {
    if m.len() <= *e as usize {
      m.resize((e + 1) as usize, !0);
    }
    m[*e as usize] = i as u32;
  }
    
  let mut cur = *v.last().unwrap();
  m[cur as usize] = !0;
  for i in v.len()..30000000 {
    if m.len() <= cur as usize {
      m.resize((cur + 1) as usize, !0);
    }
    let p = m[cur as usize];
    let next = if p == !0 { 0 } else { i as u32 - 1 - p };
    m[cur as usize] = (i - 1) as u32;
    cur = next;
  }

  println!("{}", cur);
}