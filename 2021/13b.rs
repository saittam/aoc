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
    
  for (o, d) in &folds {
    let flip = |v| if v > *d { 2 * d - v } else { v };
    p = p.iter()
      .map(|(x, y)| if *o { (flip(*x), *y) } 
                    else { (*x, flip(*y)) })
      .collect();
  }
  
  let xmin = *p.iter().map(|(x, _)| x).min().unwrap();
  let xmax = *p.iter().map(|(x, _)| x).max().unwrap();
  let ymin = *p.iter().map(|(_, y)| y).min().unwrap();
  let ymax = *p.iter().map(|(_, y)| y).max().unwrap();
  println!("{}", (ymin..=ymax).map(|y| (xmin..=xmax)
      .map(|x| if p.contains(&(x, y)) { '#' } else { ' ' })
      .collect::<String>())
    .collect::<Vec<_>>().join("\n"));
}
