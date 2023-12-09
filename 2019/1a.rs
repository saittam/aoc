use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  let mut seq = std::vec::Vec::<usize>::new();
    
    loop {
      let mut buf = String::new();
      handle.read_line(&mut buf);
      
      if buf.trim().len() == 0 {
        break
      }
      //println!("buf: {}", buf);
      seq.push(buf.trim().parse::<usize>().unwrap());
    }
    
    let mut total = 0;
    for f in seq {
      total += f / 3 - 2;
    }
    
    println!("{}", total);
}