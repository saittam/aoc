use std::io::BufRead;
use std::collections::BTreeSet;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let map = lines
    .enumerate()
    .flat_map(
      |(y, l)| l.chars().enumerate()
        .filter_map(|(x, c)|
                    if c == '#' { Some((x, y)) } else { None })
        .collect::<Vec<_>>())
    .collect::<HashSet<_>>();

  let cx = map.iter().map(|(x, _)| x).collect::<BTreeSet<_>>();
  let cy = map.iter().map(|(_, y)| y).collect::<BTreeSet<_>>();

  let map = map.iter()
    .map(|(x, y)| (2 * x - cx.range(0..*x).count(),
                   2 * y - cy.range(0..*y).count()))
    .collect::<HashSet<_>>();

  let mut mi = map.iter();
  let n = map.iter()
    .flat_map(|a| {
      mi.next();
      mi.clone().map(move |b| (a, b))
    })
    .map(|((ax, ay), (bx, by))|
         ax.abs_diff(*bx) + ay.abs_diff(*by))
    .sum::<usize>();
  
  println!("{}", n);
}