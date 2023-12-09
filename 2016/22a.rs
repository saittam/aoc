use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let nodes = lines
    .skip(2)
    .map(|l| {
      let n = l.split(|c: char| !c.is_numeric())
        .filter_map(|w| w.parse::<u32>().ok())
        .collect::<Vec<_>>();
      ((n[0], n[1]), n[3], n[4])
    })
    .collect::<Vec<_>>();

  let n = nodes.iter()
    .map(|(np, _, na)| nodes.iter()
         .map(|(p, u, _)| *u != 0 && p != np && u <= na)
         .filter(|v| *v)
         .count())
    .sum::<usize>();
    
  println!("{}", n);
}