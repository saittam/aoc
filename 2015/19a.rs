use std::io::BufRead;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut r = Vec::new();
  for l in lines.by_ref().take_while(|l| l.len() > 0) {
    let mut t = l.split(" => ");
    r.push((t.next().unwrap().to_string(),
            t.next().unwrap().to_string()));
  }

  let s = lines.next().unwrap().chars().collect::<Vec<_>>();

  let mut g = HashSet::new();
  for i in 0..s.len() {
    for (l, o) in &r {
      let e = i + l.len();
      if e <= s.len() && s[i..e].iter().cloned().eq(l.chars()) {
        let mut ro = String::with_capacity(s.len() + o.len());
        ro.extend(s[0..i].iter());
        ro.extend(o.chars());
        ro.extend(s[e..].iter());
        g.insert(ro);
      }
    }
  }
  
  println!("{}", g.len());
}