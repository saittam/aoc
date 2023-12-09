use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
    
  let mut buf = String::new(); 
  handle.read_line(&mut buf);
  
  let mut bi = buf.split('-');
  let lb = bi.next().unwrap().trim();
  let ub = bi.next().unwrap().trim();
  
  let lower = lb.parse::<usize>().unwrap();
  let upper = ub.parse::<usize>().unwrap();
  
  let mut digits = lb.chars()
    .map(|d| d.to_digit(10).unwrap())
    .collect::<Vec<u32>>();

  let mut cnt = 0;
  for _i in lower .. upper + 1 {
    let mut double = false;
    let mut asc = true;
    for i in 0 .. digits.len() - 1 {
      if digits[i] == digits[i + 1] {
        double = true;
      }
      
      if digits[i] > digits[i + 1] {
        asc = false;
      }
    }
    
    if double && asc {
      //println!("m {:?}", digits);
      cnt += 1;
    }
    
    for d in (0 .. digits.len()).rev() {
      digits[d] += 1;
      if digits[d] <= 9 {
        break;
      }
      digits[d] = 0;
    }
  }

  println!("{}", cnt);
}