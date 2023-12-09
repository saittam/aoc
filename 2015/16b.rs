use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());
  
  let eq = |v: u32| { Box::new(move |n: u32| n == v) };
  let lt = |v: u32| { Box::new(move |n: u32| n < v) };
  let gt = |v: u32| { Box::new(move |n: u32| n > v) };
  let mut props: HashMap<_, Box<dyn Fn(u32) -> bool>> =
    HashMap::new();
  props.insert("children", eq(3));
  props.insert("cats", gt(7));
  props.insert("samoyeds", eq(2));
  props.insert("pomeranians", lt(7));
  props.insert("akitas", eq(0));
  props.insert("vizslas", eq(0));
  props.insert("goldfish", lt(5));
  props.insert("trees", gt(3));
  props.insert("cars", eq(2));
  props.insert("perfumes", eq(1));

  for s in lines.take_while(|s| s.len() > 0) {
    let mut t = s.split(' ')
      .map(|s| s.trim_matches(|c:char| !c.is_alphanumeric()));
    t.next();
    let n = t.next().unwrap();
    let mut m = true;
    while let (Some(p), Some(v)) = (t.next(), t.next()) {
      m &= props.get(&p)
                .map(|p| p(v.parse::<u32>().unwrap()))
                .unwrap_or(false);
    }
    if m {
      println!("{}", n);
    }
  }
}