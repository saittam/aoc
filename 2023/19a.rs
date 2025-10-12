use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let idx = |c| ['x', 'm', 'a', 's'].iter()
    .position(|x| *x == c)
    .expect("pos");
  
  let workflows = lines.by_ref().take_while(|l| !l.is_empty())
    .map(|l| {
      let mut wi = l
        .split(|c| c == '{' || c == ',' || c == '}')
        .filter(|w| !w.is_empty());
      let name = wi.next().expect("name").to_owned();
      let mut wi = wi.rev();
      let default = wi.next().expect("default").to_owned();
      let rules = wi.rev()
        .map(|w| {
          let mut pi = w.split(':');
          let mut ci = pi.next().expect("cond").chars();
          let cat = idx(ci.next().expect("cat"));
          let op = match ci.next().expect("op") {
            '<' => i64::lt,
            '>' => i64::gt,
            o => panic!("bad op {}", o),
          };
          let arg = ci.as_str().parse::<i64>().expect("arg");
          let target = pi.next().expect("target").to_owned();
          (cat, op, arg, target)
        })
        .collect::<Vec<_>>();
      (name, (rules, default))
    })
    .collect::<HashMap<_, _>>();

  let parts = lines
    .map(|l| {
      l.split(|c| c == '{' || c == ',' || c == '}')
        .filter(|w| !w.is_empty())
        .map(|w| {
          let mut pi = w.split(|c| c == '=');
          let cat = pi.next().expect("cat")
            .chars().next().expect("char");
          let rating = pi.next().expect("rating")
            .parse::<i64>().expect("num");
          (cat, rating)
        })
        .fold([0; 4], |mut a, (c, r)| { a[idx(c)] = r; a })
    })
    .collect::<Vec<_>>();

  let n = parts.iter()
    .filter(|p| {
      std::iter::successors(Some("in"), |name| { 
        let (rules, default) = workflows.get(*name)?;
        Some(rules.iter()
        .find(|(c, o, a, _)| o(&p[*c], a))
        .map(|(_, _, _, t)| t)
        .unwrap_or(default))
      }).last().expect("last") == "A"
    })
    .map(|p| p.iter().sum::<i64>())
    .sum::<i64>();

  println!("{}", n);
}