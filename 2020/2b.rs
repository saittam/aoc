use std::io::BufRead;

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
    
    let p = buf.split(|c| !char::is_alphanumeric(c))
      .map(|s| s.to_string())
      .collect::<Vec<String>>();
    //println!("{:?}", p);
    v.push(((p[0].parse::<usize>().unwrap(),
             p[1].parse::<usize>().unwrap()),
            p[2].chars().next().unwrap(),
            p[4].clone()));
  }
  
  let mut valid = 0;
  for ((p1, p2), c, pw) in v {
    let nc = pw.chars().filter(|pc| *pc == c).count();
    if (pw.chars().nth(p1 - 1).unwrap_or(' ') == c) ^ 
       (pw.chars().nth(p2 - 1).unwrap_or(' ') == c) {
      valid += 1;
    }
  }
  
  println!("{:?}", valid);
}