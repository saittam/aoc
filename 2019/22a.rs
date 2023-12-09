use std::io::BufRead;
//use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Shuffle {
  New,
  Cut(isize),
  Incr(usize),
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut shf = Vec::new();
    
  loop {
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }
    //println!("buf: {}", buf);
    let v = buf.split(' ').map(|c| c.trim()).collect::<Vec<&str>>();
    
    //println!("{:?}", v);
    let s = if v[0] == "cut" {
      Shuffle::Cut(v[1].parse::<isize>().unwrap())
    } else if v[2] == "new" {
      Shuffle::New
    } else if v[2] == "increment" {
      Shuffle::Incr(v[3].parse::<usize>().unwrap())
    } else {
      panic!("input {}", buf);
    };

    shf.push(s);
  }
  
  //println!("{:?}", shf);
  
  let mut vfwd = vec![0; 10];
  let mut vrev = vec![0; 10];
  for i in 0..10 {
    vfwd[fwd(&shf, 10, i)] = i;
    vrev[i] = rev(&shf, 10, i);
  }
  
  //println!("{:?}", vfwd);
  //println!("{:?}", vrev);
    
  println!("{}", fwd(&shf, 10007, 2019));
}

fn fwd(shf: &[Shuffle], l: usize, idx: usize) -> usize {
  let mut i = idx;
  for s in shf.iter() {
    i = match s {
      Shuffle::New => l - 1 - i,
      Shuffle::Cut(c) => ((i + l) as isize - c) as usize,
      Shuffle::Incr(n) => i * n,
    } % l;
  }
  
  i
}
fn rev(shf: &[Shuffle], l: usize, idx: usize) -> usize {
  let mut i = idx;
  for s in shf.iter().rev() {
    i = match s {
      Shuffle::New => l - 1 - i,
      Shuffle::Cut(c) => ((i + l) as isize + c) as usize,
      Shuffle::Incr(n) => i * modinv(*n, l),
    } % l;
  }
  
  i
}

fn modinv(a: usize, n: usize) -> usize {
  let mut t = (0, 1);
  let mut r = (n as isize, a as isize);
  
  while r.1 != 0 {
    let q = r.0 / r.1;
    t = (t.1, t.0 - q * t.1);
    r = (r.1, r.0 - q * r.1);
  }

  if t.0 < 0 {
    t.0 += n as isize;
  }
  
  t.0 as usize
}