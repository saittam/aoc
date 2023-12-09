use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  const D: usize = 100;
  let mut v = vec![false; D + 2];
  for s in lines.take_while(|s| s.len() > 0) {
    v.push(false);
    v.extend(s.chars().map(|c| c == '#'));
    v.push(false);
  }
  v.extend([false; D + 2].iter());
  
  for _ in 0..100 {
    let mut nv = vec![false; v.len()];
    for y in 1..(D + 1) {
      for p in (y * (D + 2) + 1)..(y * (D + 2) + D + 1) {
        let c = [ p - D - 3, p - D - 2, p - D - 1,
                  p - 1,                p + 1,
                  p + D + 1, p + D + 2, p + D + 3 ]
          .iter().map(|i| v[*i] as usize).sum::<usize>();
        nv[p] = (c == 3) || (v[p] && c == 2);
      }
    }
    v = nv;
  }
  
  println!("{}", v.iter().filter(|l| **l).count());
}