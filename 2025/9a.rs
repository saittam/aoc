use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let pos = lines
    .map(|l| {
      let mut ni = l
        .splitn(2, ',')
        .map(|w| w.parse::<i64>().expect("num"));
      (ni.next().expect("x"), ni.next().expect("y"))
    })
    .collect::<Vec<_>>();

  let n = pos
    .iter()
    .enumerate()
    .flat_map(|(i, p1)| {
      pos.iter().skip(i + 1).map(move |p2| (p1, p2))
    })
    .map(|((x1, y1), (x2, y2))| {
      ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1)
    })
    .max()
    .expect("max");

  println!("{n}");
}
