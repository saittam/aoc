use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut names = HashMap::new();
  let mut pd = HashMap::new();
  for s in lines.take_while(|s| s.len() > 0) {
    let w = s.split(&[' ', '.'][..]).collect::<Vec<_>>();
    let mut d = w[3].parse::<isize>().unwrap();
    if w[2] == "lose" {
      d = -d;
    }
    let mut id = |c: &str| {
      let n = 1 << names.len();
      *names.entry(c.to_string()).or_insert(n)
    };
    *pd.entry(id(w[0]) | id(w[10])).or_insert(0) += d;
  }

  let mut dn1 = HashMap::new();
  for i in 1..names.len() {
    let s = (1 << i) | 1;
    if let Some(d) = pd.get(&s) {
      dn1.insert((i, s), (*d, i));
    }
  }
  let mut dn = vec![dn1];
  
  for _ in 2..names.len() {
    let mut dnn = HashMap::new();
    {
      let dnc = dn.last().unwrap();
      for e in 1..names.len() {
        let eid = 1 << e;
        let ti = dnc.iter().filter(|((_, s), _)| s & eid == 0);
        for ((n, s), (d, _)) in ti { 
          if let Some(h) = pd.get(&(eid | (1 << n))) {
            let ne = (h + d, *n);
            dnn.entry((e, s | eid))
               .and_modify(|e| *e = std::cmp::max(*e, ne))
               .or_insert(ne);
          }
        }
      }
    }
    dn.push(dnn);
  }
  
  let d = dn.last().unwrap().iter()
    .filter_map(|((n, _), (d, _))|
                pd.get(&((1 << *n) | 1)).map(|d1| d1 + d))
    .max().unwrap();
  println!("{}", d);
}