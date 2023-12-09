use std::io::BufRead;

struct Bits(Vec<u64>);

impl Bits {
  fn new(size: usize) -> Bits {
    Bits(vec![0_u64; (size + 63) / 64])
  }
  
  fn get(&self, n: usize) -> bool {
    (self.0[n / 64] & (1 << n % 64)) != 0
  }
  
  fn set(&mut self, n: usize, b: bool) {
    let s = n % 64;
    let r = &mut self.0[n / 64];
    *r = (*r & !(1 << s)) | ((b as u64) << s);
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let cubes = lines.take_while(|l| l.len() > 0)
    .map(|l| {
      let mut i = l.split_whitespace();
      let on = i.next() == Some("on");
      let n = i.next().unwrap().split(',')
        .map(|c| {
          let mut ni = c[2..]
            .split("..")
            .map(|n| n.parse::<i32>().unwrap());
          (ni.next().unwrap(), ni.next().unwrap() + 1)
        })
        .collect::<Vec<_>>();
      (on, [n[0], n[1], n[2]])
    })
    .collect::<Vec<_>>();

  let mut stops = [Vec::new(), Vec::new(), Vec::new()];
  for (_, b) in &cubes {
    for i in 0..3 {
      stops[i].push(b[i].0);
      stops[i].push(b[i].1);
    }
  }
  for s in &mut stops {
    s.sort();
    s.dedup();
  }
  
  //println!("{:?}", stops);
  
  let ls = stops.iter().map(|s| s.len()).collect::<Vec<_>>();

  println!("{:?}", ls);

  let idx = |x, y, z| x + ls[0] * (y + ls[1] * z);
  let mut s = 
    Bits::new(stops.iter().map(|s| s.len()).product());
  for (on, b) in &cubes {
    println!("{:?}", b);
    let rs = stops.iter().zip(b.iter())
      .map(|(s, (bl, bh))| {
        s.binary_search(bl).unwrap()..
        s.binary_search(bh).unwrap()
      })
      .collect::<Vec<_>>();
    println!("{:?}", rs);
    for x in rs[0].clone() {
      for y in rs[1].clone() {
        for z in rs[2].clone() {
          s.set(idx(x, y, z), *on);
        }
      }
    }
  }
  
  let mut sum = 0;
  for (kx, x) in stops[0].windows(2).enumerate() {
    for (ky, y) in stops[1].windows(2).enumerate() {
      for (kz, z) in stops[2].windows(2).enumerate() {
        if s.get(idx(kx, ky, kz)) {
          sum += (x[1] - x[0]) as u64 *
                 (y[1] - y[0]) as u64 *
                 (z[1] - z[0]) as u64;
        }
      }
    }
  }
  
  println!("{}", sum);
}