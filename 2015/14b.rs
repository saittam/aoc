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
  let mut st = vec![(0, 0, 0); v.len()];
  for t in 0..T {
    for ((d, l, p), (s, m, r)) in st.iter_mut().zip(v.iter()) {
      *d += if *l < *m { *s } else { 0 };
      *l = if *l + 1 == *m + *r { 0 } else { *l + 1 };
    }
    let max = st.iter().max().unwrap().0;
    for (d, _, p) in st.iter_mut() {
      *p += (*d == max) as u32;
    }
  }
  
  let d = st.iter().map(|(_, _, p)| p).max().unwrap();
  println!("{}", d);
}