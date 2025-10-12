use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let time = lines.next().expect("time");
  let dist = lines.next().expect("dist");
  let td = time
    .split_whitespace()
    .skip(1)
    .map(|w| w.parse::<i32>().expect("num"))
    .zip(dist.
         split_whitespace()
         .skip(1)
         .map(|w| w.parse::<i32>().expect("num")))
    .collect::<Vec<_>>();

  let n = td.iter()
    .map(|(t, d)| (0..=*t).map(|b| (t - b) * b)
                          .filter(|bd| bd > d)
                          .count())
    .product::<usize>();
 
  println!("{}", n);
}