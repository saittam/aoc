use std::io::BufRead;
use std::collections::HashMap;

fn count<'a>(
  m: &mut HashMap<(&'a str, Option<u32>, &'a [u32]), u32>,
  pat: &'a str,
  open: Option<u32>,
  seq: &'a [u32])
  -> u32 {
  let key = (pat, open, seq);
  if let Some(n) = m.get(&key) {
    return *n;
  }

  let mut pati = pat.chars();
  let patc = pati.next();
  let pat = pati.as_str();
  let onext = open.and_then(|n| n.checked_sub(1));
  let scount = |m| if let Some(n) = seq.first() {
    count(m, pat, Some(n - 1), &seq[1..])
  } else {
    0
  };
  let n = if let Some(c) = patc {
    match (c, open) {
      ('.', None) => count(m, pat, None, seq),
      ('.', Some(n)) if n == 0 => count(m, pat, None, seq),
      ('.', Some(_)) => 0,
      ('#', None) => scount(m),
      ('#', Some(n)) if n > 0 => count(m, pat, onext, seq),
      ('#', Some(_)) => 0,
      ('?', None) => count(m, pat, None, seq) + scount(m),
      ('?', Some(_)) => count(m, pat, onext, seq),
      _ => panic!("bad pattern char {}", c),
    }
  } else if open.unwrap_or(0) == 0 && seq.is_empty() {
    1
  } else {
    0
  };

  m.insert(key, n);
  n
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let rows = lines.map(|l| {
      let mut wi = l.split_whitespace();
      let pat = wi.next().expect("pattern").to_owned();
      let seq = wi.next().expect("seq").split(',')
        .map(|n| n.parse::<u32>().expect("num"))
        .collect::<Vec<_>>();
      (pat, seq)
    })
    .collect::<Vec<_>>();

  let n = rows.iter()
    .map(|(p, s)| count(&mut HashMap::new(), p, None, s))
    .sum::<u32>();
  
  println!("{}", n);
}