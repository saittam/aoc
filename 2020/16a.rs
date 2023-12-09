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
  
  let tser = tickets.iter().map(
    |t| t.iter().filter(
      |v| !fields.iter().any(
        |(_, r)| r.iter().any(
          |(min, max)| min <= *v && *v <= max)))
        .sum::<u32>())
    .sum::<u32>();

  println!("{}", tser);
}