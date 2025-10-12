use std::io::BufRead;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Constraint {
  None,
  Prev(i32),
  Bad,
}

impl Constraint {
  fn extend(&self, sig: i32, val: i32) -> Constraint {
    match self {
      Constraint::None => Constraint::Prev(val),
      Constraint::Prev(pv)
        if (1..=3).contains(&(sig * (val - pv))) =>
        Constraint::Prev(val),
      _ => Constraint::Bad,
    }
  }

  fn or(&self, other: Constraint) -> Constraint {
    match (self, other) {
      (Constraint::Bad, _) => other,
      _ => *self,
    }
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let reports = lines
    .map(|l| l.split_whitespace()
         .map(|w| w.parse::<i32>().expect("num"))
         .collect::<Vec<_>>())
    .collect::<Vec<_>>();

  // More complex than necessary, but wanted to make a 
  // solution that only iterates each report once. Idea
  // is to keep track of state for different casese of
  // order, element dropped (none, previous, earlier), 
  // separately.
  let n = reports.iter()
    .filter(
      |r| r.iter().copied().fold(
        // Array is indexed by case of inspected prefix:
        //  - safe
        //  - previous element dropped
        //  - earlier element dropped
        // for both ascending and descending order.
        // Value indicates whether the case is possible
        // and if so, what constraint to apply to next 
        // number.
        //
        // This would better be a data structure indexed
        // by (order, prefix case), but too lazy to write
        // a struct + Index impl.
        //
        // Also, the order can be detected from the first 
        // four numbers, which can be exploited to reduce
        // state array size from 6 to 3.
        [
          Constraint::None,
          Constraint::Bad,
          Constraint::Bad,
          Constraint::None,
          Constraint::Bad,
          Constraint::Bad,
        ],
        |s, v|
          [
            s[0].extend(-1, v),
            s[0],
            s[2].extend(-1, v).or(s[1].extend(-1, v)),
            s[3].extend(1, v),
            s[3],
            s[5].extend(1, v).or(s[4].extend(1, v)),
          ]
      )
      .iter()
      .any(|c| *c != Constraint::Bad))
    .count();

  println!("{n}");
}