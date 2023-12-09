use std::io::BufRead;
use std::collections::HashMap;

enum Input {
  Val(u16),
  Wire(String),
}

enum Gate {
  Pass(Input),
  Not(Input),
  And(Input, Input),
  Or(Input, Input),
  LShift(Input, Input),
  RShift(Input, Input),
}

fn eval(g: &Gate, v: &HashMap<String, u16>) -> Option<u16> {
  let r = |i: &Input| match i {
    Input::Val(n) => Some(*n),
    Input::Wire(w) => v.get(w).cloned(),
  };
  Some(match g {
    Gate::Pass(a) => r(a)?,
    Gate::Not(a) => !r(a)?,
    Gate::And(a, b) => r(a)? & r(b)?,
    Gate::Or(a, b) => r(a)? | r(b)?,
    Gate::LShift(a, b) => r(a)? << r(b)?,
    Gate::RShift(a, b) => r(a)? >> r(b)?,
  })
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut vals = HashMap::new();
  let mut gates = Vec::new();
  for s in lines.take_while(|s| s.len() > 0) {
    let pi = |w: &str| match w.parse::<u16>() {
      Ok(n) => Input::Val(n),
      Err(_) => Input::Wire(w.to_string()),
    };
    let mut t = s.split(|c: char| c.is_ascii_whitespace())
                 .collect::<Vec<_>>();
    if t[0] == "NOT" {
      gates.push((t[3].to_string(), Gate::Not(pi(t[1]))));
    } else if t[1] == "->" {
      gates.push((t[2].to_string(), Gate::Pass(pi(t[0]))));
    } else {
      let a = pi(t[0]);
      let b = pi(t[2]);
      gates.push((t[4].to_string(), match t[1] {
        "AND" => Gate::And(a, b),
        "OR" => Gate::Or(a, b),
        "LSHIFT" => Gate::LShift(a, b),
        "RSHIFT" => Gate::RShift(a, b),
        _ => panic!("Op {}", t[1]),
      }));
    }
  }
  
  loop {
    let mut gn = Vec::new();
    let gl = gates.len();
    for (w, g) in gates {
      match eval(&g, &vals) {
        Some(n) => { vals.insert(w, n); },
        None => gn.push((w, g)),
      }
    }
    
    if gn.len() == gl {
      break;
    }
    gates = gn;
  }
      
  println!("{}", vals["a"]);
}