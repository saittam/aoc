use std::io::BufRead;
use std::collections::HashMap;
use std::ops::{Add, Sub, Mul, Div};

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut nums = HashMap::new();
  let mut exprs = HashMap::new();
  for l in lines {
    let mut w = l.split(&[' ', ':']).filter(|w| w.len() > 0);
    let label = w.next().expect("label").to_string();
    let arg1 = w.next().expect("arg1");
    if let Ok(v) = arg1.parse::<usize>() {
      nums.insert(label, v);
    } else {
      let op = match w.next().expect("op") {
        "+" => usize::add,
        "-" => usize::sub,
        "*" => usize::mul,
        "/" => usize::div,
        o => panic!("{o}"),
      };
      let arg2 = w.next().expect("arg2").to_string();
      exprs.insert(label, (arg1.to_string(), op, arg2));
    }
  }

  let mut progress = true;
  while progress {
    progress = false;
    for (l, (a1, op, a2)) in &exprs {
      if let (Some(v1), Some(v2)) = 
             (nums.get(a1), nums.get(a2)) {
        let (n1, n2) = (*v1, *v2);
        progress |= 
          nums.insert(l.to_string(), op(n1, n2)).is_none();
      }
    }
  }
  
  println!("{}", nums.get("root").expect("solution"));
}