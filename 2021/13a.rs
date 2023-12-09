use std::io::BufRead;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut p = lines.by_ref().take_while(|l| l.len() > 0)
    .map(|l| {
         let mut i = l.split(',')
                      .map(|s| s.parse::<i32>().unwrap());
         (i.next().unwrap(), i.next().unwrap())
    })
    .collect::<HashSet<_>>();
    
  let folds = lines.take_while(|l| l.len() > 0)
    .map(|l| {
         let mut i =
           l.split(|c: char| !c.is_alphanumeric()).skip(2);
         (i.next().unwrap() == "x",
          i.next().unwrap().parse::<i32>().unwrap())
    })
    .collect::<Vec<_>>();
    
  let (o, d) = folds[0];
  p = p.iter()
    .map(|(x, y)| 
      if o { (if *x > d { 2 * d - x } else { *x }, *y) }
      else { (*x, if *y > d { 2 * d - y } else { *y })})
    .collect();
  
  println!("{}", p.len());
}