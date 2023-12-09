use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut l = [0i8; 1000000];
  for s in lines.take_while(|s| s.len() > 0) {
    let mut t = s.split(|c: char| !c.is_ascii_alphanumeric());
    let inc = match t.next().unwrap() {
      "turn" =>
        match t.next().unwrap() {
          "on" => 1,
          "off" => -1,
          c => panic!("turn {}", c),
        },
      "toggle" => 2,
      c => panic!("{}", c),
    };
    
    let n = t.filter_map(|n| n.parse::<usize>().ok())
             .collect::<Vec<_>>();
    for y in n[1]..(n[3] + 1) {
      for x in n[0]..(n[2] + 1) {
        let r = &mut l[y * 1000 + x];
        *r = std::cmp::max(0, *r + inc);
      }
    }
  }
      
  println!("{}", l.iter().map(|v| *v as u64).sum::<u64>());
}