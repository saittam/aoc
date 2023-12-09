use std::io::BufRead;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let map = lines
    .map(|l| {
      let mut p = l
        .split(',')
        .map(|w| w.parse::<isize>().expect("num"));
      (p.next().expect("x"),
       p.next().expect("y"),
       p.next().expect("z"))
    })
    .collect::<HashSet<_>>();
  
  let s = map.iter()
    .map(|(x, y, z)| {
      let (x, y, z) = (*x, *y, *z);
      let n = [
        (x - 1, y, z), (x + 1, y, z),
        (x, y - 1, z), (x, y + 1, z),
        (x, y, z - 1), (x, y, z + 1),
      ];
      n.iter().filter(|p| !map.contains(*p)).count()
    })
    .sum::<usize>();

  println!("{s}");
}