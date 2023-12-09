use std::io::BufRead;
//use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Shuffle {
  New,
  Cut(i128),
  Incr(u128),
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
      Shuffle::Cut(v[1].parse::<i128>().unwrap())
    } else if v[2] == "new" {
      Shuffle::New
    } else if v[2] == "increment" {
      let inc = v[3].parse::<u128>().unwrap();
      Shuffle::Incr(inc)
    } else {
      panic!("input {}", buf);
    };

    shf.push(s);
  }
  
  const MOD: u128 = 119315717514047;
  const REP: u128 = 101741582076661;
  
  let (shift, scale) = simplify(&shf, MOD);
  //println!("shift {} scale {}", shift, scale);
  //println!("{}", (2019 * scale + shift) % MOD);
  
  let scexp = exp(scale, REP, MOD);
  let shsum = ((scexp - 1) * modinv(scale - 1, MOD)) % MOD;
  let num = ((2020 + MOD - ((shsum * shift) % MOD)) * modinv(scexp, MOD)) % MOD;
  
  println!("{}", num);
}

fn simplify(shf: &[Shuffle], l: u128) -> (u128, u128) {
  let (mut shift, mut scale) = (0, 1);
  for s in shf {
    let (nsh, nsc) = match s {
      Shuffle::New => 
        (shift * (l - 1) + (l - 1), scale * (l - 1)),
      Shuffle::Cut(c) =>
        (((shift + l) as i128 - c) as u128, scale),
      Shuffle::Incr(n) =>
        (shift * n, scale * n),
    };
    shift = nsh % l;
    scale = nsc % l;
    //println!("{} {}", shift, scale);
  }
  
  (shift, scale)
}

fn modinv(a: u128, n: u128) -> u128 {
  let mut t = (0, 1);
  let mut r = (n as i128, a as i128);
  
  while r.1 != 0 {
    let q = r.0 / r.1;
    t = (t.1, t.0 - q * t.1);
    r = (r.1, r.0 - q * r.1);
  }

  if t.0 < 0 {
    t.0 += n as i128;
  }
  
  t.0 as u128
}

fn exp(b: u128, e: u128, n: u128) -> u128 {
  let mut r = 1;
  let mut re = e;
  let mut f = b;
  while re != 0 {
    if re & 1 == 1 {
      r = (r * f) % n;
    }
    f = (f * f) % n;
    re >>= 1;
  }
  r
}