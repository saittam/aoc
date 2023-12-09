use std::io::BufRead;
use std::collections::VecDeque;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let salt = lines.next().expect("salt");

  let init =
    std::iter::from_fn(|| Some((VecDeque::new(), None)))
    .take(16)
    .collect::<Vec<_>>();
  let idx = (0..)
    .map(|n| (n, (0..2016).fold(
      md5::compute(format!("{}{}", salt, n)),
      |h, _| md5::compute(format!("{:x}", h)))))
    .scan(init, |pos, (n, hash)| {
      let (c3, m5, _, _) = hash.iter()
        .flat_map(|b| [b >> 4, b & 0xf])
        .fold((None, 0, 0, 0x10), |(c3, m5, n, lc), c| {
          let n = if c == lc { n + 1 } else { 1 };
          let c3 = c3.or(if n == 3 { Some(c) } else { None });
          let m5 = m5 | ((n == 5) as usize) << c;
          (c3, m5, n, c)
        });
        
      let mut key = false;
      let k = n - 1000;
      for (i, (p3, p5)) in pos.iter_mut().enumerate() {
        if *p5 == Some(k) {
          *p5 = None;
        }
        if p3.front() == Some(&k) {
          p3.pop_front();
          if p5.is_some() {
            key = true;
          }
        }
        
        if c3 == Some(i as u8) {
          p3.push_back(n);
        }
        if m5 & (1 << i) != 0 {
          *p5 = Some(n);
        }
      }
      Some((k, key))
    })
    .filter_map(|(n, key)| if key { Some(n) } else { None })
    .nth(64 - 1)
    .expect("idx");

  println!("{}", idx);
}