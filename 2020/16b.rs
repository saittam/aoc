use std::io::BufRead;

fn main() {
  let mut stdin = std::io::stdin();
  let mut lines = stdin.lock().lines();
  
  let mut fields = Vec::new();
  loop {  
    let mut line = lines.next().unwrap().unwrap();
    if line.len() == 0 {
      break;
    }
    
    let mut kv = line.splitn(2, ':');
    let key = kv.next().unwrap().trim();
    let mut ranges = Vec::new();
    for v in kv.next().unwrap().trim().split(' ').step_by(2) {
      let mut r = v.splitn(2, '-')
        .map(|s| s.trim().parse::<u32>().unwrap());
      ranges.push((r.next().unwrap(), r.next().unwrap()));
    }
    
    fields.push((key.to_string(), ranges));
  }
      
  lines.next();
  let ticket = lines.next().unwrap().unwrap().trim().split(',')
    .map(|s| s.parse::<u32>().unwrap())
    .collect::<Vec<u32>>();
  
  lines.next();
  lines.next();
  let mut tickets = Vec::new();
  loop {
    let line = lines.next().unwrap().unwrap();
    if line.len() == 0 {
      break;
    }
    
    let ticket = line.split(',')
      .map(|s| s.parse::<u32>().unwrap())
      .collect::<Vec<u32>>();
    tickets.push(ticket);
  }
  
  let mut valid = tickets.iter()
    .filter(|t| t.iter()
      .all(|v| fields.iter()
        .any(|(_, r)| r.iter()
          .any(|(min, max)| *min <= *v && *v <= *max))))
    .cloned()
    .collect::<Vec<Vec<u32>>>();
  valid.push(ticket.clone());
    
  let mut compatible = fields.iter()
    .map(|(n, r)| (n, (0..ticket.len())
      .map(|i| valid.iter()
        .all(|t| r.iter()
          .any(|(l, h)| *l <= t[i] && t[i] <= *h)))
      .collect::<Vec<bool>>()))
    .collect::<Vec<_>>();
    
  compatible.sort_by_key(
    |(_, v)| v.iter().filter(|x| **x).count());

  let mut solutions = Vec::new();
  let mut map = vec![0; fields.len()];
  let mut i = 0;
'l:
  loop {
    if compatible[i].1[map[i]] {
      if i == fields.len() - 1 {
        solutions.push(map.clone());
        map[i] += 1;
      } else {
        i += 1;
        map[i] = 0;
      }
    } else {
      map[i] += 1;
    }
    
    loop {
      while map[0..i].contains(&map[i]) {
        map[i] += 1;
      }
      if map[i] < fields.len() {
        break;
      }
      if i == 0 {
        break 'l;
      }
      i -= 1;
      map[i] += 1;
    }
  }
  
  let s = solutions.first().unwrap();
  let p = compatible.iter().enumerate()
    .filter(|(_, (n, _))| n.starts_with("departure"))
    .map(|(i, _)| ticket[s[i]] as u64)
    .product::<u64>();
  println!("{}", p);
}