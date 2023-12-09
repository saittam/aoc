use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut c = lines.next().unwrap().chars()
                   .map(|c| c.to_digit(10).unwrap() as i32)
                   .collect::<Vec<_>>();
  let maxv = *c.iter().max().unwrap();
  let minv = *c.iter().min().unwrap();
               
  for _i in 0..100 {
    let cur = c[0];
    let mut target = cur - 1;
    loop {
      if target < minv {
        target = maxv;
      }
      if !c[0..4].iter().any(|e| *e == target) {
        break;
      }
      target -= 1;
    }
    let tpos = c[4..c.len()].iter()
      .position(|e| *e == (target))
      .unwrap() + 4;
    
    let mut nc = Vec::with_capacity(c.len());
    nc.extend(&c[4..(tpos + 1)]);
    nc.extend(&c[1..4]);
    nc.extend(&c[(tpos + 1)..c.len()]);
    nc.push(cur);
    
    c = nc;
  }
  
  let p = c.iter().position(|e| *e == 1).unwrap();
  let s = c[(p + 1)..c.len()].iter().chain(c[0..p].iter())
    .map(|e| std::char::from_digit(*e as u32, 10).unwrap())
    .collect::<String>();
  println!("{}", s);
}