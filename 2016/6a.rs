use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let msg = lines
    .fold(Vec::new(), |mut fv, w| {
      fv.resize_with(usize::max(fv.len(), w.len()),
                     HashMap::new);
      for (i, c) in w.chars().enumerate() {
        *fv[i].entry(c).or_insert(0) += 1;
      }
      fv
    })
    .iter()
    .map(|f| f.iter().max_by_key(|(_, c)| *c).expect("max"))
    .map(|(c, _)| *c)
    .collect::<String>();

  println!("{}", msg);
}