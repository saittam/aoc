use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

fn balance<'a>(id: &'a str,
               m: &'a HashMap<String, (u32, Vec<String>)>)
  -> Result<u32, (&'a str, u32)> {
  let (w, rec) = &m[id];
  let rw = rec.iter()
    .map(|r| balance(r, m))
    .fold(Ok(None), |m, rw| {
      let rv = rw?;
      let ov = m?.unwrap_or(rv);
      if rv == ov {
        Ok(Some(rv))
      } else {
        // Note that if there are only two siblings with
        // differing weights, it is not possible to decide
        // locally what the correct fix is. So, just report
        // back the id and delta and just try all possible
        // fixes to identify the right one.
        Err((id, u32::abs_diff(ov, rv)))
      }
    })?;
  Ok(w + rec.len() as u32 * rw.unwrap_or(0))
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut m = lines
    .map(|l| {
      let mut wi = l.split_whitespace();
      let id = wi.next().expect("id").to_owned();
      let weight = wi.next().expect("weight")
        .trim_matches(['(', ')'].as_slice())
        .parse::<u32>()
        .expect("num");
      wi.next();
      let rec = wi
        .map(|w| w.trim_matches(',').to_owned())
        .collect::<Vec<_>>();
      (id, (weight, rec))
    })
    .collect::<HashMap<_, _>>();

  let ids = m.keys().collect::<HashSet<_>>();
  let rids = m.values()
    .flat_map(|(_, rec)| rec)
    .collect::<HashSet<_>>();
  let root = ids.difference(&rids)
    .next().expect("root").to_string();

  let (id, delta) = balance(&root, &m).expect_err("delta");
  for rec in &m[id].1.clone() {
    let w = m[rec].0;
    for wp in [w - delta, w + delta] {
      m.get_mut(rec).unwrap().0 = wp;
      if balance(&root, &m).is_ok() {
        println!("{}", wp);
      }
    }
    m.get_mut(rec).unwrap().0 = w;
  }
}