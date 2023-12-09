use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut names = HashMap::new();
  let mut pd = HashMap::new();
  for s in lines.take_while(|s| s.len() > 0) {
    let w = s.split(' ').collect::<Vec<_>>();
    let d = w[4].parse::<usize>().unwrap();
    let mut id = |c: &str| {
      let n = 1 << names.len();
      *names.entry(c.to_string()).or_insert(n)
    };
    pd.insert(id(w[0]) | id(w[2]), d);
  }
  
  let mut dn1 = HashMap::new();
  for i in 0..names.len() {
    dn1.insert((i, 1 << i), (0, i));
  }
  let mut dn = vec![dn1];
  
  for _ in 1..names.len() {
    let mut dnn = HashMap::new();
    {
      let dnc = dn.last().unwrap();
      for e in 0..names.len() {
        let eid = 1 << e;
        let ti = dnc.iter().filter(|((_, s), _)| s & eid == 0);
        for ((n, s), (d, _)) in ti { 
          if let Some(h) = pd.get(&(eid | (1 << n))) {
            let ne = (h + d, *n);
            dnn.entry((e, s | eid))
               .and_modify(|e| *e = std::cmp::min(*e, ne))
               .or_insert(ne);
          }
        }
      }
    }
    dn.push(dnn);
  }
  
  let (_, (d, _)) = dn.last().unwrap().iter()
                      .min_by_key(|(_, (d, _))| d)
                      .unwrap();
  println!("{}", d);
}