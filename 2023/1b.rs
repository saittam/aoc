use std::io::BufRead;

const PATTERNS: [(&'static str, u32); 19] = [
  ("0", 0),
  ("1", 1),
  ("2", 2),
  ("3", 3),
  ("4", 4),
  ("5", 5),
  ("6", 6),
  ("7", 7),
  ("8", 8),
  ("9", 9),
  ("one", 1),
  ("two", 2),
  ("three", 3),
  ("four", 4),
  ("five", 5),
  ("six", 6),
  ("seven", 7),
  ("eight", 8),
  ("nine", 9),
];

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let n = lines
    .map(|l| PATTERNS
         .iter()
         .flat_map(|(p, v)| l.match_indices(p)
                             .map(move |k| (k, v)))
         .fold(None,
               |mm, e| mm.or(Some((e, e)))
                         .map(|(f, l)| (f.min(e), l.max(e))))
         .map(|((_, f), (_, l))| f * 10 + l)
         .expect("minmax"))
    .sum::<u32>();
  println!("{}", n);
}