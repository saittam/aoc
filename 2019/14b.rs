use std::io::BufRead;

type Quant = (usize, String);
type IList = Vec<Quant>;

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut r = Vec::<(Quant, IList)>::new();
  loop {
    let mut buf = String::new();
    handle.read_line(&mut buf);
    
    if buf.trim().len() == 0 {
      break;
    }

    let mut bi = buf.split("=>");
    
    fn pq(s: &str) -> Quant {
      let mut ii = s.trim().split(' ');
      let n = ii.next().unwrap().trim().parse::<usize>().unwrap();
      let c = ii.next().unwrap().trim();
      (n, c.to_string())
    };
      
    let inlist = bi.next().unwrap().split(',').map(pq).collect::<IList>();
    let q = pq(bi.next().unwrap());
    
    r.push((q, inlist));
  }
  
  let mut seen  = std::collections::HashSet::<String>::new();
  seen.insert("ORE".to_string());
  
  for i in 0..r.len() {
    let mut s = r.len();
    for j in i..r.len() {
      let ((_, ref c), ref l) = r[j];
      if l.iter().all(|(_, c)| seen.contains(c)) {
        s = j;
        seen.insert(c.clone());
        break;
      }
    }
    r.swap(i, s);
  }
  
  let mut ub = 1;
  loop {
    let o = ore(&r, ub);
    //println!("{} -> {:?}", ub, o);
    if o > (1000000000000) {
      break;
    }
    ub *= 2;
  }
  
  let mut lb = ub / 2;
  while lb + 1 < ub {
    let t = (lb + ub) / 2;
    let o = ore(&r, t);
    //println!("{} -> {:?}", t, o);
    if o > (1000000000000) {
      ub = t;
    } else {
      lb = t;
    }
  }
  
  println!("{}", lb);
}

fn ore(r: &Vec<(Quant, IList)>, n: usize) -> usize {
  let mut want = std::collections::HashMap::<String, usize>::new();
  want.insert("FUEL".to_string(), n);
  for ((n, c), l) in r.iter().rev() {
    let wn = *want.get(c).unwrap_or(&0);
    let f = (wn + n - 1) / n;
    for (ni, ci) in l {
      *want.entry(ci.clone()).or_insert(0) += f * ni;
    }
  }
  
  want["ORE"]
}