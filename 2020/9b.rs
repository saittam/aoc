use std::io::BufRead;
use std::cmp::Ordering;

fn find_pair(v: &[u64], s: u64) -> Option<()> {
  let mut fwd = v.iter();
  let mut rev = v.iter().rev();
  
  let mut a = fwd.next().unwrap();
  let mut b = rev.next().unwrap();
  
  while a <= b {
    match (a + b).cmp(&s) {
      Ordering::Less => a = fwd.next()?,
      Ordering::Equal => return Some(()),
      Ordering::Greater => b = rev.next()?, 
    }
  }
  
  None
}

fn find_sum(v: &[u64], s: u64) -> Option<u64> {
  let mut headi = v.iter().skip(1);
  let mut taili = v.iter();
  
  let mut headn = *headi.next().unwrap();
  let mut tailn = *taili.next().unwrap();
  let mut sum = headn + tailn;
  
  loop {
    match sum.cmp(&s) {
      Ordering::Less => {
        headn = *headi.next()?;
        sum += headn;
      },
      Ordering::Equal => break,
      Ordering::Greater => {
        sum -= tailn;
        tailn = *taili.next()?;
      },
    }
  }
  
  let mut mm = (tailn, tailn);
  sum -= tailn;
  while sum > 0 {
    tailn = *taili.next().unwrap();
    mm = (u64::min(mm.0, tailn), u64::max(mm.1, tailn));
    sum -= tailn;
  }
  
  Some(mm.0 + mm.1)
    
}

fn update(v: &mut [u64], drop: u64, insert: u64) {
  let mut pw = 0;
  let mut pr = 0;
  
  let mut odrop = Some(drop);
  let mut oinsert = Some(insert);

  let mut val = v[pr];
  pr += 1;
  loop {
    if let Some(i) = oinsert {
      if i <= val {
        v[pw] = i;
        pw += 1;
        oinsert = None;
      }
    }
    
    if pr >= v.len() {
      break;
    }
    
    if Some(val) == odrop {
      val = v[pr];
      pr += 1;
      odrop = None;
      continue;
    }
        
    let valn = v[pr];
    pr += 1;
    v[pw] = val;
    pw += 1;
    val = valn;
  }
  
  if Some(val) != odrop {
    v[pw] = val;
    pw += 1;
  }
  
  if let Some(i) = oinsert {
    v[pw] = i;
  }
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut v = Vec::new();
  loop {  
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }
    
    v.push(buf.trim().parse::<u64>().unwrap());
  }
  
  let plen = 25;
  let mut c = v[0..plen].to_vec();
  c.sort();
  
  for (d, n) in v.iter().zip(v.iter().skip(plen)) {
    if find_pair(&c, *n).is_none() {
      println!("{:?}", find_sum(&v, *n).unwrap());
      break;
    }
    update(&mut c, *d, *n);
  }
}