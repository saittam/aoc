use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let pos = lines
    .map(|l| {
      let mut ni = l
        .splitn(3, ',')
        .map(|w| w.parse::<i64>().expect("num"));
      (
        ni.next().expect("x"),
        ni.next().expect("y"),
        ni.next().expect("z"),
      )
    })
    .collect::<Vec<_>>();

  let mut dist = (0..pos.len())
    .flat_map(|i1| {
      ((i1 + 1)..pos.len()).map(move |i2| (i1, i2))
    })
    .collect::<Vec<_>>();
  dist.sort_by_key(|(i1, i2)| {
    let (x1, y1, z1) = pos[*i1];
    let (x2, y2, z2) = pos[*i2];
    (x1 - x2).abs().pow(2)
      + (y1 - y2).abs().pow(2)
      + (z1 - z2).abs().pow(2)
  });

  let n = dist
    .iter()
    .scan(
      (pos.len(), (0..pos.len()).collect::<Vec<_>>()),
      |(ngroups, groups), (i1, i2)| {
        let g1 = groups[*i1];
        let g2 = groups[*i2];
        if g1 != g2 {
          *ngroups -= 1;
          for i in 0..pos.len() {
            if groups[i] == g2 {
              groups[i] = g1;
            }
          }
        }
        Some((*ngroups, (i1, i2)))
      },
    )
    .find_map(|(ngroups, (i1, i2))| {
      (ngroups == 1).then(|| pos[*i1].0 * pos[*i2].0)
    })
    .unwrap();

  println!("{n}");
}
