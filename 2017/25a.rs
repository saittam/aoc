use std::io::BufRead;
use std::collections::HashMap;

fn last_word(l: Option<String>) -> String {
  l.expect("line")
   .split(|c: char| !c.is_alphanumeric())
   .filter(|w| !w.is_empty())
   .last()
   .expect("last")
   .to_string()
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut nextid = 0;
  let mut idmap = HashMap::new();
  let mut getid = move |s| *idmap.entry(s).or_insert_with(
    || { nextid += 1; nextid - 1 });

  let start = getid(last_word(lines.next()));
  let steps = lines.next().expect("line")
    .split(|c: char| !c.is_alphanumeric())
    .find_map(|w| w.parse::<usize>().ok())
    .expect("steps");
  lines.next();

  let mut rules = HashMap::new();
  while let Some(r) = lines.next() {
    let state = getid(last_word(Some(r)));
    rules.extend(
      (0..2)
      .map(|_| {
        let tape = last_word(lines.next())
          .parse::<u32>().expect("tape") != 0;
        let write = last_word(lines.next())
          .parse::<u32>().expect("write") != 0;
        let dir = match last_word(lines.next()).as_str() {
          "left" => -1,
          "right" => 1,
          _ => panic!("bad dir"),
        };
        let next = getid(last_word(lines.next()));
        ((state, tape), (write, next, dir))
      }));
    lines.next();
  }
    
  let mut tape = HashMap::new();
  let mut cursor = 0;
  let mut state = start;
  for _ in 0..steps {
    let val = tape.entry(cursor).or_insert(false);
    let (nval, nstate, dir) = rules[&(state, *val)];
    *val = nval;
    state = nstate;
    cursor += dir;
  }

  let count = tape.values().filter(|b| **b).count();
  println!("{}", count);
}
