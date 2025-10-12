use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut bricks = lines
    .map(|l| {
      let nv = l.split(|c: char| !c.is_numeric())
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<i64>().expect("num"))
        .collect::<Vec<_>>();
      assert_eq!(nv.len(), 6);
      ((nv[0], nv[1], nv[2]), (nv[3], nv[4], nv[5]))
    })
    .collect::<Vec<_>>();
      
  bricks.sort_by_key(
    |((_, _, z1), (_, _, z2))| i64::min(*z1, *z2));

  let mut supp = vec![HashSet::new(); bricks.len()];

  let _hm = bricks.iter().enumerate().fold(
    HashMap::new(),
    |mut hm, (i, ((x1, y1, z1), (x2, y2, z2)))| {
      let (dx, dy, dz) = (x2 - x1, y2 - y1, z2 - z1);
      let d = dx + dy + dz;
      let (sx, sy, sz) =
        (dx.signum(), dy.signum(), dz.signum());
      let (h, _) = *(0..=d)
        .map(|i| (x1 + i * sx, y1 + i * sy))
        .map(|p| hm.get(&p).unwrap_or(&(0, 0)))
        .max()
        .expect("h");

      for k in 0..=d {
        let (x, y, z) =
          (x1 + k * sx, y1 + k * sy, h + 1 + k * sz);
        hm.entry((x, y))
          .and_modify(|(eh, ei)| {
            if *eh == z - 1 && *ei != i {
              supp[i].insert(*ei);
            }
            *eh = z;
            *ei = i;
          })
          .or_insert((z, i));
      }

      hm
    });
  
  let required = supp.iter()
    .filter_map(
      |s| if s.len() == 1 { s.iter().next() }else { None })
    .collect::<HashSet<_>>();
  let n = bricks.len() - required.len();

  println!("{}", n);
}