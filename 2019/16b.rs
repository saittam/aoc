use std::io::BufRead;

fn show(v: &[isize]) {
  println!("{}", v.iter().map(
    |i| std::char::from_u32((i.abs() % 10) as u32 + '0' as u32).unwrap())
    .collect::<String>());
}

fn correct(vi: &[isize]) {
  let mut v = Vec::from(vi);
    
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
  
  show(&v);
}

fn matrix(vi: &[isize]) {
  let mut v = Vec::from(vi);

  let pat = [0, 1, 0, -1];
  let mut pv = Vec::<Vec<isize>>::new();
  for i in 1..=v.len() {
    pv.push(pat.iter().flat_map(|e| vec![*e; i]).cycle().skip(1).take(v.len()).collect());
  }
  
  let mut m = pv.clone();
  for _ in 0..99 {
    let mut mn = Vec::<Vec<isize>>::new();
    for r in pv.iter() {
      let mut rn = Vec::<isize>::new();
      for j in 0..pv.len() {
        rn.push(r.iter().enumerate().map(|(i, v)| v * m[i][j]).sum::<isize>() % 10);
      }
      mn.push(rn);
    }
    
    m = mn;
  }
  
  for r in &m {
    show(&r);
  }
  println!("");
  
  let t = m.iter().map(|r| r.iter().zip(v.iter()).map(
    |(rv, vv)| rv * vv).sum::<isize>().abs() % 10).collect::<Vec<isize>>();
  show(&t);
}

fn digit(n: usize, iv: &[isize], rep: usize) -> isize {
  assert!(rep % 4 == 0);
  let v = iv.iter().cloned().cycle().take(iv.len() * 4).collect::<Vec<isize>>();
  
  let mut it = v.iter().cloned().skip(n % v.len());
  let mut r = it.next().unwrap();
  r += it.zip([0, 0, 0, 5].iter().cycle()).map(
    |(vv, vp)| vv * vp).sum::<isize>() % 10;
  
  let ph = (v.len() - ((n % v.len()) + 1)) % 4;
  let d = [0, 0, 0, 5].iter().cycle().skip(ph).zip(v.iter()).map(
    |(vv, vp)| vv * vp).sum::<isize>() % 10;
  
  //println!("{} {} {}", (rep / 4), n, v.len());
  let t = (rep / 4) - (n / v.len()) - 1;
  
  //println!("ph {} t {}", ph, t);
    
  (r + t as isize * d) % 10
}

struct TrIter {
  s: [isize; 100],
}

impl TrIter {
  fn new() -> TrIter {
    TrIter { s: [1; 100] }
  }
}

impl Iterator for TrIter {
  type Item = isize;
  
  fn next(&mut self) -> Option<Self::Item> {
    let r = self.s[99];
    for i in 1..100 {
      self.s[i] = (self.s[i] + self.s[i - 1]) % 10;
    }
    Some(r)
  }
}

fn trmat(n: usize, v: &[isize], rep: usize) -> Vec<isize> {
  let mut it = v.iter().cloned().cycle();
  let mut its = Vec::new();
  for i in 0..8 {
    its.push(it.clone().skip((n % v.len()) + i));
  }
  
  let mut tri = TrIter::new();
  let mut r = vec![0; its.len()];
  for i in n..(v.len() * rep - r.len()) {
    let t = tri.next().unwrap();
    for k in 0..r.len() {
      r[k] = (r[k] + t * its[k].next().unwrap()) % 10;
    }
  }
  
  for i in 1..r.len() {
    let t = tri.next().unwrap();
    for k in 0..(r.len() - i) {
      r[k] = (r[k] + t * its[k].next().unwrap()) % 10;
    }
  }
  
  r
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();

  let mut buf = String::new();
  handle.read_line(&mut buf);
      
  let mut v = buf.trim().chars()
    .map(|c| c.to_digit(10).unwrap() as isize)
    .collect::<Vec<isize>>();

  const REP: usize = 10000;
  
  /*
  let vm = v.iter().cloned().cycle().take(REP * v.len()).collect::<Vec<isize>>();
  
  matrix(&vm);
  correct(&vm);
  */
    
  let mut offset = v.iter().take(7).fold(0, |n, d| n * 10 + d);
  //println!("{:?} {}", &v[0..7], offset);
  
  let r = trmat(offset as usize, &v, REP);
  show(&r);
  
  //show(&(0..(v.len() * REP)).into_iter().map(|n| digit(n, &v, REP)).collect::<Vec<isize>>());
  
  //println!("{}% rem {}", (offset as usize * 100) / (10000 * v.len()), 10000 * v.len() - offset as usize);
  
  /*
  println!("{}", (0..8).into_iter().map(
    |n| digit((n + offset) as usize, &v, 10000))
    .fold(0, |n, d| n * 10 + d));
  */
}