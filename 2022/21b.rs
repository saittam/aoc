use std::io::BufRead;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Op {
  Add, Sub, Mul, Div,
}

enum Job {
  Num(usize),
  Expr(Box<Job>, Op, Box<Job>),
  Human,
}

impl Job {
  fn from(l: &str,
          nums: &HashMap<String, usize>,
          exprs: &HashMap<String, (String, Op, String)>)
    -> Option<Job> {
    if l == "humn" {
      Some(Job::Human)
    } else if let Some(n) = nums.get(l) {
      Some(Job::Num(*n))
    } else if let Some((r1, op, r2)) = exprs.get(l) {
      Some(Job::Expr(Box::new(Job::from(r1, nums, exprs)?),
                     *op,
                     Box::new(Job::from(r2, nums, exprs)?)))
    } else {
      None
    }
  }
  
  fn simplify(&self) -> Job {
    match self {
      Job::Human => Job::Human,
      Job::Num(v) => Job::Num(*v),
      Job::Expr(r1, op, r2) => {
        let (s1, s2) = (r1.simplify(), r2.simplify());
        if let (Job::Num(v1), Job::Num(v2)) = (&s1, &s2) {
          Job::Num(match op {
            Op::Add => v1 + v2,
            Op::Sub => v1 - v2,
            Op::Mul => v1 * v2,
            Op::Div => v1 / v2,
          })
        } else {
          Job::Expr(Box::new(s1), *op, Box::new(s2))
        }
      }
    }
  }

  fn propagate(&self, v: usize) -> usize {
    match self {
      Job::Human => v,
      Job::Expr(j1, op, j2) => 
        match (j1.as_ref(), j2.as_ref()) {
          (Job::Num(n), r) => r.propagate(match op {
              Op::Add => v - n,
              Op::Sub => n - v,
              Op::Mul => v / n,
              Op::Div => n / v,
            }),
          (r, Job::Num(n)) => r.propagate(match op {
              Op::Add => v - n,
              Op::Sub => v + n,
              Op::Mul => v / n,
              Op::Div => v * n,
            }),
          _ => panic!("propagate"),
        }
      _ => panic!("propagate"),
    }
  }
}

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
        "+" => Op::Add,
        "-" => Op::Sub,
        "*" => Op::Mul,
        "/" => Op::Div,
        o => panic!("{o}"),
      };
      let arg2 = w.next().expect("arg2").to_string();
      exprs.insert(label, (arg1.to_string(), op, arg2));
    }
  }

  let root = Job::from("root", &nums, &exprs).expect("root");
  let n = if let Job::Expr(j1, _, j2) = root.simplify() {
    match (*j1, *j2) {
      (Job::Num(n), r) | (r, Job::Num(n)) => r.propagate(n),
      _ => panic!("root shape"),
    }
  } else {
    panic!("root shape")
  };
  println!("{n}");
}