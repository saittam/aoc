use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let insns = lines
    .map(|l| {
      let mut wi = l.split_whitespace();
      let reg = wi.next().expect("reg").to_string();
      let op = wi.next().expect("op");
      let amount = wi.next().expect("amount")
        .parse::<i32>().expect("num");
      let amount = match op {
        "inc" => amount,
        "dec" => -amount,
        o => panic!("bad op {}", o),
      };
      wi.next();
      let creg = wi.next().expect("reg").to_string();
      let comp = match wi.next().expect("comp") {
        "<" => i32::lt,
        "<=" => i32::le,
        "==" => i32::eq,
        "!=" => i32::ne,
        ">" => i32::gt,
        ">=" => i32::ge,
        c => panic!("bad comp {}", c),
      };
      let cval = wi.next().expect("amount")
        .parse::<i32>().expect("num");
      (reg, amount, creg, comp, cval)
    })
    .collect::<Vec<_>>();

  let mut regs = HashMap::new();
  for (reg, amount, creg, comp, cval) in &insns {
    if comp(regs.get(creg).unwrap_or(&0), cval) {
      *regs.entry(reg).or_insert(0) += amount;
    }
  }

  let max = regs.values().max().expect("max");
  println!("{}", max);
}