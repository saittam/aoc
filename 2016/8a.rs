use std::io::BufRead;
use std::collections::HashSet;

enum Op {
  Rect(u32, u32),
  RotRow(u32, u32),
  RotCol(u32, u32),
}

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let ops = lines.map(|l| {
    let nums = l.split(|c: char| !c.is_digit(10))
                .filter(|w| w.len() > 0)
                .map(|w| w.parse::<u32>().expect("num"))
                .collect::<Vec<_>>();
    let words = l.split_whitespace().collect::<Vec<_>>();
    match (words[0], words[1]) {
      ("rect", _) => Op::Rect(nums[0], nums[1]),
      ("rotate", "row") => Op::RotRow(nums[0], nums[1]),
      ("rotate", "column") => Op::RotCol(nums[0], nums[1]),
      _ => panic!("bad op"),
    }
  })
  .collect::<Vec<_>>();

  const W: u32 = 50;
  const H: u32 = 6;
  let pixels = ops.iter()
    .fold(HashSet::new(), |mut pixels, op| match op {
      Op::Rect(w, h) => {
        pixels.extend(
          (0..*w).flat_map(|x| (0..*h).map(move |y| (x, y))));
        pixels
      }
      Op::RotRow(ry, k) => pixels.into_iter()
        .map(|(x, y)| if *ry == y { ((x + k) % W, y) }
                      else { (x, y) })
        .collect::<HashSet<_>>(),
      Op::RotCol(cx, k) => pixels.into_iter()
        .map(|(x, y)| if *cx == x { (x, (y + k) % H) }
                      else { (x, y) })
        .collect::<HashSet<_>>(),
    });

  println!("{}", pixels.len());
}