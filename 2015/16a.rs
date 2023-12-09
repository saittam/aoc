use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let mut props = HashMap::new();
  props.insert("children", 3);
  props.insert("cats", 7);
  props.insert("pomeranians", 3);
  props.insert("akitas", 0);
  props.insert("vizslas", 0);
  props.insert("goldfish", 5);
  props.insert("trees", 3);
  props.insert("cars", 2);
  props.insert("perfumes", 1);

  for s in lines.take_while(|s| s.len() > 0) {
    let mut t = s.split(' ')
      .map(|s| s.trim_matches(|c:char| !c.is_alphanumeric()));
    t.next();
    let n = t.next().unwrap();
    let mut m = true;
    while let (Some(p), Some(v)) = (t.next(), t.next()) {
      m &= props.get(&p) == Some(&v.parse::<u32>().unwrap());
    }
    if m {
      println!("{}", n);
    }
  }
}