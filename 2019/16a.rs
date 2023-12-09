use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
    
  let mut buf = String::new();
  handle.read_line(&mut buf);
      
  let mut v = buf.trim().chars()
    .map(|c| c.to_digit(10).unwrap() as isize)
    .collect::<Vec<isize>>();
  
  let pat = [0, 1, 0, -1];
  let mut pv = Vec::<Vec<isize>>::new();
  for i in 1..=v.len() {
    pv.push(pat.iter().flat_map(|e| vec![*e; i]).cycle().skip(1).take(v.len()).collect());
  }
  
  for _ in 0..100 {
    let mut vn = pv.iter().map(|p| {
        let s: isize = p.iter().zip(v.iter()).map(
          |(vp, vv)| vp * vv).sum();
        s.abs() % 10
      }).collect();
    v = vn;
    
    //println!("{:?}", v);
  }
  
  println!("{}", v.iter().map(|d| d.to_string()).take(8).collect::<String>());
}