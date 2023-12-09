use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  
  let mut m = Vec::new();
  loop {
    let mut d = String::new();
    loop {
      let mut buf = String::new();
      handle.read_line(&mut buf);
    
      if buf.trim().len() == 0 {
        break;
      }
      d += &buf;
    }
    
    if d.trim().len() == 0 {
      break;
    }
    
    let r = d.split(char::is_whitespace)
      .filter(|s| s.len() > 0)
      .map(|s| s.split(':'))
      .map(|mut a| (a.next().unwrap().to_string(),
                    a.next().unwrap().to_string()))
      .collect::<HashMap<String, String>>();
    m.push(r);
  }
  
  let req = [ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" ];
  let valid = m.iter()
    .filter(|r| req.iter().all(|k| r.contains_key(*k)))
    .count();
    
  println!("{:?}", valid);
}